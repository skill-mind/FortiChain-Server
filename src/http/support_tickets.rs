use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use sqlx::prelude::*;

use super::types::{AssignSupportTicketRequest, OpenSupportTicketRequest};
use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/open_ticket", post(open_ticket_handler))
        .route("/assign_ticket", post(assign_ticket_handler))
    //   .route("/close_ticket", post(close_ticket_handler))
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
pub async fn assign_ticket_handler(
    state: State<AppState>,
    Json(payload): Json<AssignSupportTicketRequest>,
) -> StatusCode {
    let db = &state.db;
    // 1. Check ticket exists and is not already assigned
    let ticket_query = r#"
        SELECT status::TEXT, assigned_to FROM request_ticket WHERE id = $1;
    "#;
    let ticket_row = match db
        .pool
        .fetch_one(sqlx::query(ticket_query).bind(payload.ticket_id))
        .await
    {
        Ok(row) => row,
        Err(e) => {
            tracing::error!(ticket_id = %payload.ticket_id, error = ?e, "Ticket not found or DB error");
            return StatusCode::NOT_FOUND;
        }
    };
    let status: String = match ticket_row.try_get("status") {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(ticket_id = %payload.ticket_id, error = ?e, "Failed to extract status from ticket_row");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    // let assigned_to: Option<String> = ticket_row.try_get("assigned_to").ok();
    if status == "assigned" || status == "in_progress" {
        tracing::warn!(ticket_id = %payload.ticket_id, status = %status, "Ticket already assigned or in progress");
        return StatusCode::CONFLICT; // Already assigned
    }
    // 2. Check agent exists and is a support_agent
    let agent_query = r#"
        SELECT type::TEXT FROM escrow_users WHERE wallet_address = $1
    "#;
    let agent_row = match db
        .pool
        .fetch_one(sqlx::query(agent_query).bind(&payload.support_agent_wallet))
        .await
    {
        Ok(row) => row,
        Err(e) => {
            tracing::error!(support_agent_wallet = %payload.support_agent_wallet, error = ?e, "Support agent not found or DB error");
            return StatusCode::BAD_REQUEST; // Agent not found
        }
    };
    let agent_type: String = match agent_row.try_get("type") {
        Ok(t) => t,
        Err(e) => {
            tracing::error!(support_agent_wallet = %payload.support_agent_wallet, error = ?e, "Failed to extract type from agent_row");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    if agent_type != "support_agent" {
        tracing::warn!(support_agent_wallet = %payload.support_agent_wallet, agent_type = %agent_type, "User is not a support agent");
        return StatusCode::FORBIDDEN;
    }
    // 3. Prevent assignment to unavailable agents (already assigned to another open/assigned/in_progress ticket)
    let agent_busy_query = r#"
        SELECT 1 FROM request_ticket WHERE assigned_to = $1 AND status IN ('assigned', 'in_progress', 'open')
    "#;
    let agent_busy = match db
        .pool
        .fetch_optional(sqlx::query(agent_busy_query).bind(&payload.support_agent_wallet))
        .await
    {
        Ok(opt) => opt.is_some(),
        Err(e) => {
            tracing::error!(support_agent_wallet = %payload.support_agent_wallet, error = ?e, "Failed to check if agent is busy");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    if agent_busy {
        tracing::warn!(support_agent_wallet = %payload.support_agent_wallet, "Agent is already assigned to another open/assigned/in_progress ticket");
        return StatusCode::CONFLICT; // Agent is busy
    }
    // 4. Update ticket
    let update_query = r#"
        UPDATE request_ticket SET status = 'assigned', assigned_to = $1, updated_at = NOW() WHERE id = $2
    "#;
    match db
        .pool
        .execute(
            sqlx::query(update_query)
                .bind(&payload.support_agent_wallet)
                .bind(payload.ticket_id),
        )
        .await
    {
        Ok(_) => {
            tracing::info!(ticket_id = %payload.ticket_id, support_agent_wallet = %payload.support_agent_wallet, "Ticket successfully assigned");
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!(ticket_id = %payload.ticket_id, support_agent_wallet = %payload.support_agent_wallet, error = ?e, "Failed to update ticket assignment");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
