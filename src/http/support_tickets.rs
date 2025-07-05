use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use sqlx::prelude::*;

use super::types::OpenSupportTicketRequest;
use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/open_ticket", post(open_ticket_handler))
    //   .route("/close_ticket", post(close_ticket_handler))
    //   .route("/assign_ticket", post(assign_ticket_handler))
    //   .route("/unassign_ticket", post(unassign_ticket_handler))
}

#[tracing::instrument(skip(state, payload))]
async fn open_ticket_handler(
    state: State<AppState>,
    Json(payload): Json<OpenSupportTicketRequest>,
) -> axum::http::StatusCode {
    // Validate the payload
    let subject_len = payload.subject.trim().len();
    let message_len = payload.message.trim().len();
    if !(5..=99).contains(&subject_len)
        || !(10..=4999).contains(&message_len)
        || payload.opened_by.trim().is_empty()
    {
        return StatusCode::BAD_REQUEST;
    }

    tracing::info!(opened_by = %payload.opened_by, subject = %payload.subject, "Attempting to create a support ticket");

    // Check if the user exists in the escrow_users table
    let db = &state.db;
    let user_exists_query = r#"
        SELECT EXISTS(
            SELECT 1 FROM escrow_users WHERE wallet_address = $1
        )
    "#;

    let user_exists: bool = match db
        .pool
        .fetch_one(sqlx::query(user_exists_query).bind(&payload.opened_by))
        .await
    {
        Ok(row) => match row.try_get::<bool, _>(0) {
            Ok(val) => val,
            Err(e) => {
                tracing::error!("Failed to extract boolean from row: {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        },
        Err(e) => {
            tracing::error!("Failed to check user existence: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if !user_exists {
        tracing::error!("User does not exist in escrow_users table");
        return StatusCode::BAD_REQUEST;
    }

    // Logic to open a ticket
    let db = &state.db;
    let query = r#"
        INSERT INTO request_ticket (
            subject,
            message,
            opened_by,
            status,
            response_subject
        ) VALUES ($1, $2, $3, $4::ticket_status_type, $5)
    "#;

    if let Err(e) = db
        .pool
        .execute(
            sqlx::query(query)
                .bind(payload.subject.clone())
                .bind(payload.message)
                .bind(payload.opened_by)
                .bind("open")
                .bind(payload.subject),
        )
        .await
    {
        tracing::error!("Failed to insert ticket: {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::CREATED
}
