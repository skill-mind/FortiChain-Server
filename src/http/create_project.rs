use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use uuid::Uuid;

use super::types::CreateProjectRequest;
use crate::AppState;

#[derive(Debug, serde::Serialize)]
pub struct ApiResponse {
    pub message: String,
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/create_project", post(create_project_handler))
}

#[tracing::instrument(name = "create_project", skip(state, payload))]
async fn create_project_handler(
    state: State<AppState>,
    Json(payload): Json<CreateProjectRequest>,
) -> impl IntoResponse {
    let (status, msg) = create_project_handler_inner(state, payload).await;
    let body = Json(ApiResponse { message: msg });
    (status, body)
}

// Move the main logic here, signature matches previous handler
async fn create_project_handler_inner(
    state: State<AppState>,
    payload: CreateProjectRequest,
) -> (StatusCode, String) {
    // --- Validation ---
    // Validate required fields
    if payload.owner_address.trim().is_empty()
        || payload.contract_address.trim().is_empty()
        || payload.name.trim().is_empty()
        || payload.description.trim().is_empty()
        || payload.contact_info.trim().is_empty()
    {
        return (
            StatusCode::BAD_REQUEST,
            "Missing required fields".to_string(),
        );
    }

    // Validate address format (0x + 64 hex chars)
    let is_valid_addr = |addr: &str| {
        addr.starts_with("0x")
            && addr.len() == 66
            && addr.chars().skip(2).all(|c| c.is_ascii_hexdigit())
    };
    if !is_valid_addr(&payload.owner_address) || !is_valid_addr(&payload.contract_address) {
        return (
            StatusCode::BAD_REQUEST,
            "Invalid address format".to_string(),
        );
    }

    // Validate bounty fields: all or none
    let bounty_amount_present = payload.bounty_amount.is_some();
    let bounty_currency_present = payload.bounty_currency.is_some();
    let bounty_expiry_present = payload.bounty_expiry_date.is_some();
    let filled = [
        bounty_amount_present,
        bounty_currency_present,
        bounty_expiry_present,
    ]
    .iter()
    .filter(|&&f| f)
    .count();
    if filled != 0 && filled != 3 {
        return (
            StatusCode::BAD_REQUEST,
            "Bounty fields must be all present or all null".to_string(),
        );
    }

    // Validate contact_info (basic email or url check)
    let is_email = |s: &str| s.contains('@') && s.contains('.') && !s.contains(' ');
    let is_url = |s: &str| s.starts_with("http://") || s.starts_with("https://");
    if !is_email(&payload.contact_info) && !is_url(&payload.contact_info) {
        return (
            StatusCode::BAD_REQUEST,
            "Invalid contact_info format".to_string(),
        );
    }

    // --- DB Transaction ---
    let db = &state.db;
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            let msg = format!("Failed to start transaction: {e:?}");
            tracing::error!("{}", msg);
            return (StatusCode::INTERNAL_SERVER_ERROR, msg);
        }
    };

    // Insert project and get id as string, then parse to Uuid
    let project_id_str: Result<String, sqlx::Error> = sqlx::query_scalar(
        r#"
        INSERT INTO projects (
            owner_address, contract_address, name, description, contact_info,
            supporting_document_path, project_logo_path, repository_url,
            bounty_amount, bounty_currency, bounty_expiry_date
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id::text
    "#,
    )
    .bind(&payload.owner_address)
    .bind(&payload.contract_address)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.contact_info)
    .bind(&payload.supporting_document_path)
    .bind(&payload.project_logo_path)
    .bind(&payload.repository_url)
    .bind(&payload.bounty_amount)
    .bind(&payload.bounty_currency)
    .bind(payload.bounty_expiry_date)
    .fetch_one(&mut *tx)
    .await;

    let project_id = match project_id_str {
        Ok(id_str) => match Uuid::parse_str(&id_str) {
            Ok(uuid) => uuid,
            Err(e) => {
                let msg = format!("Failed to parse project id as Uuid: {e:?}");
                tracing::error!("{}", msg);
                let _ = tx.rollback().await;
                return (StatusCode::INTERNAL_SERVER_ERROR, msg);
            }
        },
        Err(e) => {
            // Map DB constraint violations to 400, unique to 409, else 500
            if let Some(db_err) = e.as_database_error() {
                let code_owned = db_err.code().map(|c| c.to_string());
                let code = code_owned.as_deref();
                match code {
                    Some("23505") => {
                        // unique_violation
                        let _ = tx.rollback().await;
                        return (
                            StatusCode::CONFLICT,
                            format!("Unique constraint violation: {}", db_err.message()),
                        );
                    }
                    Some("23514") | Some("23502") | Some("22001") | Some("22007")
                    | Some("22008") | Some("22003") | Some("22004") | Some("22012") => {
                        let _ = tx.rollback().await;
                        return (
                            StatusCode::BAD_REQUEST,
                            format!("Constraint violation: {}", db_err.message()),
                        );
                    }
                    Some("23513") => {
                        // exclusion_violation
                        let _ = tx.rollback().await;
                        return (
                            StatusCode::BAD_REQUEST,
                            format!("Exclusion violation: {}", db_err.message()),
                        );
                    }
                    Some("23503") => {
                        // foreign_key_violation
                        let _ = tx.rollback().await;
                        return (
                            StatusCode::BAD_REQUEST,
                            format!("Foreign key violation: {}", db_err.message()),
                        );
                    }
                    _ => {}
                }
            }
            let msg = format!("Failed to insert project: {e:?}");
            tracing::error!("{}", msg);
            let _ = tx.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, msg);
        }
    };

    // Upsert tags and collect their IDs
    let mut tag_ids = Vec::new();
    for tag in &payload.tags {
        let tag_id: Result<i32, sqlx::Error> = sqlx::query_scalar(
            r#"
            INSERT INTO tags (name) VALUES ($1)
            ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
            RETURNING id
        "#,
        )
        .bind(tag)
        .fetch_one(&mut *tx)
        .await;
        match tag_id {
            Ok(id) => tag_ids.push(id),
            Err(e) => {
                let msg = format!("Failed to upsert tag '{tag}': {e:?}");
                tracing::error!("{}", msg);
                let _ = tx.rollback().await;
                return (StatusCode::INTERNAL_SERVER_ERROR, msg);
            }
        }
    }

    // Insert into project_tags
    for tag_id in tag_ids {
        if let Err(e) = sqlx::query(
            r#"
            INSERT INTO project_tags (project_id, tag_id) VALUES ($1, $2)
            ON CONFLICT DO NOTHING
        "#,
        )
        .bind(project_id)
        .bind(tag_id)
        .execute(&mut *tx)
        .await
        {
            let msg = format!("Failed to associate tag with project: {e:?}");
            tracing::error!("{}", msg);
            let _ = tx.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, msg);
        }
    }

    if let Err(e) = tx.commit().await {
        let msg = format!("Failed to commit transaction: {e:?}");
        tracing::error!("{}", msg);
        return (StatusCode::INTERNAL_SERVER_ERROR, msg);
    }

    (
        StatusCode::CREATED,
        "Project created successfully".to_string(),
    )
}
