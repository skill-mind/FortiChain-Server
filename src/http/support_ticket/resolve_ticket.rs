use crate::{AppState, Error, Result, http::support_ticket::ResolveSupportTicketRequest};
use axum::{Json, extract::State, http::StatusCode};
use garde::Validate;

#[tracing::instrument(name = "resolve_ticket_handler", skip(state, payload))]
pub async fn resolve_ticket_handler(
    state: State<AppState>,
    Json(payload): Json<ResolveSupportTicketRequest>,
) -> Result<StatusCode> {
    payload.validate()?;
    let db = &state.db;
    let mut tx = db.pool.begin().await?;

    tracing::info!(
        ticket_id = %payload.ticket_id,
        "Attempting to resolve support ticket"
    );

    let (status, _assigned_to): (String, Option<String>) = match sqlx::query_as(
        r#"
        SELECT status::TEXT, assigned_to
        FROM request_ticket
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(payload.ticket_id)
    .fetch_optional(&mut *tx)
    .await?
    {
        Some(row) => row,
        None => {
            tracing::warn!(
                ticket_id = %payload.ticket_id,
                "Ticket not found for resolution"
            );
            return Err(Error::NotFound);
        }
    };

    if status == "resolved" {
        tracing::warn!(
            ticket_id = %payload.ticket_id,
            "Ticket already resolved"
        );
        return Err(Error::Conflict);
    }

    let resolver_type: String = match sqlx::query_scalar(
        r#"
        SELECT type::TEXT
        FROM escrow_users
        WHERE wallet_address = $1
        "#,
    )
    .bind(&payload.resolved_by)
    .fetch_optional(&mut *tx)
    .await?
    {
        Some(resolver_type) => resolver_type,
        None => {
            tracing::warn!(
                resolver = %payload.resolved_by,
                "Resolver user not found"
            );
            return Err(Error::NotFound);
        }
    };

    if resolver_type != "admin" {
        tracing::warn!(
            resolver = %payload.resolved_by,
            resolver_type = %resolver_type,
            "Only admins can resolve tickets"
        );
        return Err(Error::Forbidden);
    }

    let rows_updated = sqlx::query!(
        r#"
        UPDATE request_ticket
        SET
            status = 'resolved'::ticket_status_type,
            resolution_response = $1,
            resolved_at = NOW(),
            updated_at = NOW()
        WHERE id = $2
        "#,
        &payload.resolution_response,
        payload.ticket_id
    )
    .execute(&mut *tx)
    .await?
    .rows_affected();

    if rows_updated == 0 {
        tracing::error!(
            ticket_id = %payload.ticket_id,
            "No rows updated during resolution"
        );
        return Err(Error::InternalServerError(anyhow::anyhow!(
            "ticket not assigned"
        )));
    }

    tx.commit().await?;
    tracing::info!(
        ticket_id = %payload.ticket_id,
        "Ticket successfully resolved"
    );

    Ok(StatusCode::OK)
}
