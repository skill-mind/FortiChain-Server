use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use sqlx::prelude::*;
use uuid::Uuid;

use super::types::{
    AssignSupportTicketRequest, OpenSupportTicketRequest, ResolveSupportTicketRequest,
};
use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/open_ticket", post(open_ticket_handler))
        .route("/assign_ticket", post(assign_ticket_handler))
        .route("/resolve_ticket", post(resolve_ticket_handler))
    //   .route("/close_ticket", post(close_ticket_handler))
    //   .route("/unassign_ticket", post(unassign_ticket_handler))
}

#[tracing::instrument(name = "open_ticket_handler", skip(state, payload))]
async fn open_ticket_handler(
    state: State<AppState>,
    Json(payload): Json<OpenSupportTicketRequest>,
) -> axum::http::StatusCode {
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

    let db = &state.db;

    tracing::info!("Query to confirm the user opening a ticket exists");
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

    tracing::info!(opened_by = %payload.opened_by, subject = %payload.subject, "Attempting to create a support ticket");
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

    tracing::info!("Support ticket successfully created.");
    StatusCode::CREATED
}

#[tracing::instrument(name = "assign_ticket_handler", skip(state, payload))]
pub async fn assign_ticket_handler(
    state: State<AppState>,
    Json(payload): Json<AssignSupportTicketRequest>,
) -> StatusCode {
    let db = &state.db;
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

    if status == "assigned" || status == "in_progress" {
        tracing::warn!(ticket_id = %payload.ticket_id, status = %status, "Ticket already assigned or in progress");
        return StatusCode::CONFLICT;
    }

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
            return StatusCode::BAD_REQUEST;
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
        tracing::info!(support_agent_wallet = %payload.support_agent_wallet, agent_type = %agent_type, "User is not a support agent");
        return StatusCode::FORBIDDEN;
    }

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
        tracing::info!(support_agent_wallet = %payload.support_agent_wallet, "Agent is already assigned to another open/assigned/in_progress ticket");
        return StatusCode::CONFLICT;
    }

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

#[tracing::instrument(name = "Resolve Ticket", skip(state, payload))]
async fn resolve_ticket_handler(
    state: State<AppState>,
    Json(payload): Json<ResolveSupportTicketRequest>,
) -> axum::http::StatusCode {
    let resolution_response_len = payload.resolution_response.trim().len();
    if resolution_response_len < 10 {
        tracing::error!(
            resolution_response_len = resolution_response_len,
            "Resolution response too short (min 10 characters)"
        );
        return StatusCode::BAD_REQUEST;
    }

    let db = &state.db;

    tracing::info!(
        ticket_id = %payload.ticket_id,
        resolved_by = %payload.resolved_by,
        "Attempt to resolve support ticket"
    );

    let ticket_uuid = match Uuid::parse_str(&payload.ticket_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            tracing::error!("Invalid UUID format: {}", payload.ticket_id);
            return StatusCode::BAD_REQUEST;
        }
    };

    let ticket_query = r#"
    SELECT status::TEXT, assigned_to, opened_by
    FROM request_ticket
    WHERE id = $1
    "#;

    let ticket_row = match sqlx::query(ticket_query)
        .bind(ticket_uuid)
        .fetch_one(&db.pool)
        .await
    {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("Ticket not found with id {}: {:?}", payload.ticket_id, e);
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

    let resolver_query = r#"
        SELECT type::TEXT FROM escrow_users WHERE wallet_address = $1
    "#;
    let resolver_row = match db
        .pool
        .fetch_one(sqlx::query(resolver_query).bind(&payload.resolved_by))
        .await
    {
        Ok(row) => row,
        Err(_) => {
            tracing::error!("Resolver not found in escrow_users table");
            return StatusCode::BAD_REQUEST;
        }
    };

    let resolver_type: String = match resolver_row.try_get("type") {
        Ok(t) => t,
        Err(_) => {
            tracing::error!("Failed to extract resolver type");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if resolver_type != "admin" {
        tracing::error!("Only user admin type can resolve support tickets");
        return StatusCode::FORBIDDEN;
    }

    let resolve_query = r#"
        UPDATE request_ticket
        SET
            status = 'resolved'::ticket_status_type,
            resolution_response = $1,
            resolved_at = NOW()
        WHERE id = $3
    "#;

    match db
        .pool
        .execute(
            sqlx::query(resolve_query)
                .bind(&payload.resolution_response)
                .bind(&payload.resolved_by)
                .bind(ticket_uuid),
        )
        .await
    {
        Ok(_) => {
            tracing::info!(
                ticket_id = %ticket_uuid,
                resolved_by = %payload.resolved_by,
                "Support Ticket Resolved Successfully"
            );
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!("Failed to resolve ticket: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
