use crate::{
    AppState, Error, Result,
    http::support_ticket::{ListTicketsQuery, SupportTicket},
};
use axum::{Json, extract::Query, extract::State};
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
