use axum::{Json, extract::State, http::StatusCode};

use crate::{AppState, Error, Result, http::support_ticket::AssignSupportTicketRequest};

#[tracing::instrument(name = "assign_ticket_handler", skip(state, payload))]
pub async fn assign_ticket_handler(
    state: State<AppState>,
    Json(payload): Json<AssignSupportTicketRequest>,
) -> Result<StatusCode> {
    let db = &state.db;
    let mut tx = db.pool.begin().await?;

    tracing::info!(
        ticket_id = %payload.ticket_id,
        agent = %payload.support_agent_wallet,
        "Attempting to assign ticket"
    );

    let (status, _current_assignee): (String, Option<String>) = match sqlx::query_as(
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
            tracing::warn!(ticket_id = %payload.ticket_id, "Ticket not found");
            return Err(Error::NotFound);
        }
    };

    if status == "assigned" || status == "in_progress" {
        tracing::warn!(
            ticket_id = %payload.ticket_id,
            status = %status,
            "Ticket already assigned or in progress"
        );
        return Err(Error::Conflict);
    }

    let agent_info: (String, bool) = match sqlx::query_as(
        r#"
        SELECT
            type::TEXT,
            EXISTS(
                SELECT 1
                FROM request_ticket
                WHERE assigned_to = $1
                AND status IN ('assigned', 'in_progress', 'open')
            ) AS is_busy
        FROM escrow_users
        WHERE wallet_address = $1
        "#,
    )
    .bind(&payload.support_agent_wallet)
    .fetch_optional(&mut *tx)
    .await?
    {
        Some(row) => row,
        None => {
            tracing::warn!(
                agent = %payload.support_agent_wallet,
                "Agent not found"
            );
            return Err(Error::NotFound);
        }
    };

    if agent_info.0 != "support_agent" {
        tracing::warn!(
            agent = %payload.support_agent_wallet,
            agent_type = %agent_info.0,
            "User is not a support agent"
        );
        return Err(Error::Forbidden);
    }

    if agent_info.1 {
        tracing::warn!(
            agent = %payload.support_agent_wallet,
            "Agent is already assigned to another ticket"
        );
        return Err(Error::Conflict);
    }

    let rows_updated = sqlx::query!(
        r#"
        UPDATE request_ticket
        SET
            status = 'assigned'::ticket_status_type,
            assigned_to = $1,
            updated_at = NOW()
        WHERE id = $2
        "#,
        &payload.support_agent_wallet,
        payload.ticket_id
    )
    .execute(&mut *tx)
    .await?
    .rows_affected();

    if rows_updated == 0 {
        tracing::error!(
            ticket_id = %payload.ticket_id,
            "No rows updated during assignment"
        );
        return Err(Error::InternalServerError(anyhow::anyhow!(
            "ticket not assigned"
        )));
    }

    tx.commit().await?;
    tracing::info!(
        ticket_id = %payload.ticket_id,
        agent = %payload.support_agent_wallet,
        "Ticket successfully assigned"
    );

    Ok(StatusCode::OK)
}
