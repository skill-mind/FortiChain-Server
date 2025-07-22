use crate::{AppState, Error, Result, http::support_ticket::OpenSupportTicketRequest};
use axum::{Json, extract::State, http::StatusCode};
use garde::Validate;

#[tracing::instrument(name = "open_ticket_handler", skip(state, payload))]
pub async fn open_ticket_handler(
    state: State<AppState>,
    Json(payload): Json<OpenSupportTicketRequest>,
) -> Result<StatusCode> {
    payload.validate()?;
    let db = &state.db;

    tracing::info!(
        opened_by = %payload.opened_by,
        subject = %payload.subject,
        "Attempting to create support ticket"
    );

    let result = sqlx::query!(
        r#"
        INSERT INTO request_ticket (
            subject,
            message,
            opened_by,
            status,
            response_subject
        )
        SELECT $1, $2, $3, 'open'::ticket_status_type, $4
        FROM escrow_users
        WHERE wallet_address = $5 -- Changed from $3 to $5
        RETURNING id
        "#,
        payload.subject,
        payload.message,
        payload.opened_by,
        payload.subject,
        payload.opened_by
    )
    .fetch_optional(&db.pool)
    .await?;

    match result {
        Some(_) => {
            tracing::info!("Support ticket successfully created");
            Ok(StatusCode::CREATED)
        }
        None => {
            tracing::error!(
                "User {} does not exist in escrow_users table",
                payload.opened_by
            );
            Err(Error::Forbidden)
        }
    }
}
