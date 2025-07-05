use axum::{extract::State, Router, http::StatusCode, routing::{get, post}, Json};
use sqlx::prelude::*; // Import Executor trait

use crate::AppState;
use crate::db::Db;
use super::types::{OpenSupportTicketRequest, SupportTicket};

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/open_ticket", post(open_ticket_handler))
                //   .route("/close_ticket", post(close_ticket_handler))
                //   .route("/assign_ticket", post(assign_ticket_handler))
                //   .route("/unassign_ticket", post(unassign_ticket_handler))
}

async fn open_ticket_handler(
    state: State<AppState>,
    Json(payload): Json<OpenSupportTicketRequest>,
) -> axum::http::StatusCode {
    // Validate the payload
    if payload.subject.is_empty() || payload.message.is_empty() || payload.opened_by.is_empty() {
        return StatusCode::BAD_REQUEST;
    }

    // Check if the user exists in the escrow_users table
    let db = &state.db;
    let user_exists_query = r#"
        SELECT EXISTS(
            SELECT 1 FROM escrow_users WHERE wallet_address = $1
        )
    "#;

    let user_exists: bool = match db.pool.fetch_one(sqlx::query(user_exists_query).bind(&payload.opened_by)).await {
        Ok(row) => match row.try_get::<bool, _>(0) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Failed to extract boolean from row: {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        },
        Err(e) => {
            eprintln!("Failed to check user existence: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if !user_exists {
        eprintln!("User does not exist in escrow_users table");
        return StatusCode::BAD_REQUEST;
    }
    


    // Logic to open a ticket
    let db = &state.db;
    let query = r#"
        INSERT INTO request_tickets (
            subject,
            message,
            opened_by
        ) VALUES ($1, $2, $3)
    "#;

    if let Err(e) = db.pool.execute(sqlx::query(query)
        .bind(payload.subject)
        .bind(payload.message)
        .bind(payload.opened_by)
    ).await {
        eprintln!("Failed to insert ticket: {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    

    StatusCode::CREATED
}
