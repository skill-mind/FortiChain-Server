use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use sqlx::prelude::*;

use super::types::{OpenSupportTicketRequest, ResolveSupportTicketRequest};
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
        tracing::error!(
            subject_len = subject_len,
            message_len = message_len,
            "Invalid payload"
        );
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

#[tracing::instrument(skip(state, payload))]
async fn resolve_ticket_handler(
    state: State<AppState>,
    Json(payload): Json<ResolveSupportTicketRequest>,
) -> axum::http::StatusCode {
    tracing::info!(
        ticket_id = %payload.ticket_id,
        resolved_by = %payload.resolved_by,
        "Attempt to resolve support ticket"
    );
    // check if ticket exist
    let ticket_query = r#"
        SELECT status::TEXT, assigned_to, opened_by FROM request_ticket WHERE id = $1
    "#;

    let ticket_row = match db
        .pool
        .fetch_one(sqlx::query(ticket_query).bind(&payload.ticket_id))
        .await
    {
        Ok(row) => row,
        Err(_) => {
            tracing::error!("Ticket not found id {}", payload.ticket_id);
            return StatusCode::NOT_FOUND;
        }
    };

    let status: String = match ticket_row.try_get("status") {
        Ok(s) => s,
        Err(_) => {
            tracing::error!("Failed to extract status from ticket_row");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if status == "resolved" {
        tracing::error!("Ticket is already resolved");
        return StatusCode::CONFLICT;
    }

    // check only admin can resolve support ticket

    // we should have field that save who resolve the ticket //   resolved_by

    let resolve_query = r#"
        UPDATE request_ticket 
        SET 
            status = 'resolved'::ticket_status_type,
            resolution_response = $1,
            resolved_at = NOW(),
            updated_at = NOW()
        WHERE id = $3
        "#;

    match db
        .pool
        .execute(
            sqlx::query(resolve_query)
                .bind(&payload.resolution_response)
                .bind(&payload.resolved_by)
                .bind(&payload.ticket_id),
        )
        .await
    {
        Ok(_) => {
            // TODO : SEND User notification
            tracing::info!(
            ticket_id = %payload.ticket_id,
            resolved_by = %payload.resolved_by,
            "Support Ticket Resolve Successfully"
            );
            StatusCode::Ok();
        }
        Err(e) => {
            tracing::error!("Failed to resolve ticket: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
