// src/http/projects.rs
use axum::{
    Router,
    extract::{Path, State},
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{AppState, Error};

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/projects/{project_id}/verify", post(verify_project))
        .route("/projects/{project_id}", get(get_project))
}

#[derive(Debug, Deserialize)]
pub struct VerifyProjectRequest {
    pub repository_url: String,
    pub owner_address: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub name: String,
    pub owner_address: String,
    pub contract_address: String,
    pub description: String,
    pub is_verified: bool,
    pub verification_date: Option<chrono::DateTime<chrono::Utc>>,
    pub repository_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct VerifyProjectResponse {
    pub message: String,
    pub project_id: Uuid,
    pub verified_at: chrono::DateTime<chrono::Utc>,
}

#[tracing::instrument(name = "Verify Project", skip(state), fields(project_id = %project_id))]
pub async fn verify_project(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Json(request): Json<VerifyProjectRequest>,
) -> Result<Json<VerifyProjectResponse>, Error> {
    // Validate repository URL format
    if !is_valid_repository_url(&request.repository_url) {
        tracing::error!(
            project_id = %project_id,
            repository_url = %request.repository_url,
            "Invalid repository URL format"
        );
        return Err(Error::unprocessable_entity([(
            "repository_url",
            "Invalid repository URL format",
        )]));
    }

    // Validate owner address format
    if !is_valid_starknet_address(&request.owner_address) {
        tracing::error!(
            project_id = %project_id,
            owner_address = %request.owner_address,
            "Invalid Starknet address format"
        );
        return Err(Error::unprocessable_entity([(
            "owner_address",
            "Invalid Starknet address format",
        )]));
    }

    // Check if project exists and get current state
    tracing::info!(
        project_id = %project_id,
        "Fetching project details for verification"
    );
    let project = match get_project_by_id(&state.db.pool, project_id).await {
        Ok(project) => {
            tracing::debug!(
                project_id = %project_id,
                is_verified = %project.is_verified,
                owner_address = %project.owner_address,
                "Retrieved project details"
            );
            project
        }
        Err(e) => {
            tracing::error!(
                project_id = %project_id,
                error = %e,
                "Failed to fetch project details"
            );
            return Err(e);
        }
    };

    // Check if the requester is the project owner
    if project.owner_address != request.owner_address {
        tracing::error!(
            project_id = %project_id,
            requester_address = %request.owner_address,
            actual_owner = %project.owner_address,
            "Verification attempted by non-owner"
        );
        return Err(Error::Forbidden);
    }

    // Check if project is already verified
    if project.is_verified {
        tracing::warn!(
            project_id = %project_id,
            "Attempt to reverify already verified project"
        );
        return Err(Error::unprocessable_entity([(
            "verification",
            "Project is already verified",
        )]));
    }

    // Perform verification
    let verification_date = chrono::Utc::now();

    match verify_project_in_db(
        &state.db.pool,
        project_id,
        &request.repository_url,
        verification_date,
    )
    .await
    {
        Ok(_) => {
            tracing::info!("Project {} successfully verified", project_id);
            Ok(Json(VerifyProjectResponse {
                message: "Project successfully verified".to_string(),
                project_id,
                verified_at: verification_date,
            }))
        }
        Err(e) => {
            tracing::error!("Failed to verify project {}: {}", project_id, e);
            Err(Error::unprocessable_entity([(
                "verification",
                "Failed to verify project",
            )]))
        }
    }
}

#[tracing::instrument(name = "Get Project", skip(state), fields(project_id = %project_id))]
pub async fn get_project(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ProjectResponse>, Error> {
    let project = get_project_by_id(&state.db.pool, project_id).await?;
    Ok(Json(project))
}

async fn get_project_by_id(pool: &PgPool, project_id: Uuid) -> Result<ProjectResponse, Error> {
    let project = sqlx::query_as!(
        ProjectResponse,
        r#"
        SELECT 
            id,
            name,
            owner_address,
            contract_address,
            description,
            is_verified,
            verification_date,
            repository_url,
            created_at
        FROM projects 
        WHERE id = $1
        "#,
        project_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error when fetching project: {}", e);
        Error::unprocessable_entity([("database", "Failed to fetch project")])
    })?;

    project.ok_or(Error::NotFound)
}

async fn verify_project_in_db(
    pool: &PgPool,
    project_id: Uuid,
    repository_url: &str,
    verification_date: chrono::DateTime<chrono::Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE projects 
        SET 
            is_verified = true,
            verification_date = $2,
            repository_url = $3
        WHERE id = $1
        "#,
        project_id,
        verification_date,
        repository_url
    )
    .execute(pool)
    .await?;

    Ok(())
}

fn is_valid_repository_url(url: &str) -> bool {
    // Basic URL validation for repository URLs
    url.starts_with("https://")
        && (url.contains("github.com")
            || url.contains("gitlab.com")
            || url.contains("bitbucket.org"))
}

fn is_valid_starknet_address(address: &str) -> bool {
    // Validate Starknet address format (0x + 64 hex characters)
    address.len() == 66
        && address.starts_with("0x")
        && address.chars().skip(2).all(|c| c.is_ascii_hexdigit())
}
