use crate::{
    AppState, Error, Result,
    http::support_ticket::{ListTicketsQuery, SupportTicket, TicketHistoryParams},
};
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
};
use garde::Validate;

/// GET /tickets?status=open,assigned&sort=asc&limit=20&offset=0
#[tracing::instrument(name = "list_tickets_handler", skip(state))]
pub async fn list_tickets_handler(
    state: State<AppState>,
    Query(params): Query<ListTicketsQuery>,
) -> Result<Json<Vec<SupportTicket>>> {
    params.validate()?;
    let db = &state.db;

    let statuses: Vec<String> = params
        .status
        .as_ref()
        .map(|s| {
            s.split(',')
                .map(|status| status.trim().to_lowercase())
                .collect()
        })
        .unwrap_or_default();

    let order_direction = params
        .sort
        .as_ref()
        .map(|s| s.to_uppercase())
        .unwrap_or_else(|| "ASC".to_string());

    let sql = format!(
        r#"
        SELECT
            id::text, subject, message, document_path, opened_by, status::text, assigned_to,
            response_subject, resolution_response, resolved, created_at::text, resolved_at::text,
            updated_at::text
        FROM request_ticket
        WHERE $1 = '{{}}' OR status::text = ANY($1)
        ORDER BY created_at {order_direction} LIMIT $2 OFFSET $3"#
    );

    tracing::info!(
        statuses = ?statuses,
        params.limit,
        params.offset,
        "Fetching support tickets with provided filters"
    );

    let rows = sqlx::query_as::<_, SupportTicket>(&sql)
        .bind(&statuses)
        .bind(params.limit.unwrap_or(10))
        .bind(params.offset.unwrap_or(0))
        .fetch_all(&db.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => e.into(),
        })?;

    Ok(Json(rows))
}

#[tracing::instrument(name = "ticket_history_handler", skip(state, params))]
async fn ticket_history_handler(
    State(state): State<AppState>,
    Query(params): Query<TicketHistoryParams>,
) -> Result<Json<Vec<super::types::SupportTicket>>, StatusCode> {
    let wallet = params.wallet.trim();
    if wallet.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = (page - 1) * limit;

    let query = r#"
        SELECT
            id::text,
            subject,
            message,
            document_path,
            opened_by,
            status::text,
            assigned_to,
            response_subject,
            resolution_response,
            resolved,
            created_at::text,
            resolved_at::text,
            updated_at::text
        FROM request_ticket
        WHERE opened_by = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    "#;

    let tickets = match sqlx::query_as::<_, super::types::SupportTicket>(query)
        .bind(wallet)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&state.db.pool)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("Failed to fetch ticket history: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(tickets))
}