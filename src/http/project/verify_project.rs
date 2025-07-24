use crate::{
    AppState, Error, Result,
    http::project::{VerifyProjectRequest, VerifyProjectResponse, shared::get_project_by_id},
};
use axum::{
    Json,
    extract::{Path, State},
};
use garde::Validate;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Verify Project", skip(state), fields(project_id = %project_id))]
pub async fn verify_project(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Json(request): Json<VerifyProjectRequest>,
) -> Result<Json<VerifyProjectResponse>> {
    request.validate()?;

    tracing::info!(
        project_id = %project_id,
        "Fetching project details for verification"
    );
    let project = get_project_by_id(&state.db.pool, project_id).await?;

    if project.owner_address != request.owner_address {
        tracing::error!(
            project_id = %project_id,
            requester_address = %request.owner_address,
            actual_owner = %project.owner_address,
            "Verification attempted by non-owner"
        );
        return Err(Error::Forbidden);
    }

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

async fn verify_project_in_db(
    pool: &PgPool,
    project_id: Uuid,
    repository_url: &str,
    verification_date: chrono::DateTime<chrono::Utc>,
) -> Result<()> {
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
