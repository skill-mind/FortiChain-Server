use crate::{AppState, Result};
use crate::http::project::{ListProjectsQuery, ListProjectsResponse, ProjectListItem};
use axum::{Json, extract::{Query, State}};
use garde::Validate;
use sqlx::Row;

#[tracing::instrument(name = "list_projects", skip(state))]
pub async fn list_projects_handler(
    State(state): State<AppState>,
    Query(params): Query<ListProjectsQuery>,
) -> Result<Json<ListProjectsResponse>> {
    params.validate()?;

    let limit = params.limit.unwrap_or(10);
    let offset = params.offset.unwrap_or(0);
    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = params.sort_order.as_deref().unwrap_or("desc");

    // Build the ORDER BY clause
    let order_by = match sort_by {
        "bounty_amount" => format!("p.bounty_amount {} NULLS LAST", sort_order.to_uppercase()),
        "name" => format!("p.name {}", sort_order.to_uppercase()),
        _ => format!("p.created_at {}", sort_order.to_uppercase()),
    };

    // Build query based on filters
    let (total_count_query, projects_query) = if let Some(_owner_address) = &params.owner_address {
        // Filter by owner address  
        let base_conditions = "WHERE p.owner_address = $1";
        let mut additional_conditions = Vec::new();
        
        if params.active_only.unwrap_or(false) {
            additional_conditions.push("p.closed_at IS NULL");
        }
        
        if params.has_bounty.unwrap_or(false) {
            additional_conditions.push("p.bounty_amount IS NOT NULL");
            additional_conditions.push("p.closed_at IS NULL");
        }
        
        let where_clause = if additional_conditions.is_empty() {
            base_conditions.to_string()
        } else {
            format!("{} AND {}", base_conditions, additional_conditions.join(" AND "))
        };

        let count_query = format!(
            "SELECT COUNT(*) FROM projects p {}",
            where_clause
        );

        let projects_query = format!(
            r#"
            SELECT 
                p.id, p.name, p.owner_address, p.contract_address, p.description,
                p.is_verified, p.verification_date, p.repository_url, p.bounty_amount,
                p.bounty_currency, p.bounty_expiry_date, p.created_at, p.closed_at,
                COALESCE(
                    array_agg(t.name ORDER BY t.name) FILTER (WHERE t.name IS NOT NULL),
                    ARRAY[]::text[]
                ) as tags
            FROM projects p
            LEFT JOIN project_tags pt ON p.id = pt.project_id
            LEFT JOIN tags t ON pt.tag_id = t.id
            {}
            GROUP BY p.id, p.name, p.owner_address, p.contract_address, p.description, 
                     p.is_verified, p.verification_date, p.repository_url, p.bounty_amount, 
                     p.bounty_currency, p.bounty_expiry_date, p.created_at, p.closed_at
            ORDER BY {}
            LIMIT $2 OFFSET $3
            "#,
            where_clause, order_by
        );

        (count_query, projects_query)
    } else {
        // No owner filter
        let mut conditions = Vec::new();
        
        if params.active_only.unwrap_or(false) {
            conditions.push("p.closed_at IS NULL");
        }
        
        if params.has_bounty.unwrap_or(false) {
            conditions.push("p.bounty_amount IS NOT NULL");
            conditions.push("p.closed_at IS NULL");
        }
        
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_query = format!(
            "SELECT COUNT(*) FROM projects p {}",
            where_clause
        );

        let projects_query = format!(
            r#"
            SELECT 
                p.id, p.name, p.owner_address, p.contract_address, p.description,
                p.is_verified, p.verification_date, p.repository_url, p.bounty_amount,
                p.bounty_currency, p.bounty_expiry_date, p.created_at, p.closed_at,
                COALESCE(
                    array_agg(t.name ORDER BY t.name) FILTER (WHERE t.name IS NOT NULL),
                    ARRAY[]::text[]
                ) as tags
            FROM projects p
            LEFT JOIN project_tags pt ON p.id = pt.project_id
            LEFT JOIN tags t ON pt.tag_id = t.id
            {}
            GROUP BY p.id, p.name, p.owner_address, p.contract_address, p.description, 
                     p.is_verified, p.verification_date, p.repository_url, p.bounty_amount, 
                     p.bounty_currency, p.bounty_expiry_date, p.created_at, p.closed_at
            ORDER BY {}
            LIMIT $1 OFFSET $2
            "#,
            where_clause, order_by
        );

        (count_query, projects_query)
    };

    // Execute count query
    let total_count: i64 = if let Some(owner_address) = &params.owner_address {
        sqlx::query_scalar(&total_count_query)
            .bind(owner_address)
            .fetch_one(&state.db.pool)
            .await?
    } else {
        sqlx::query_scalar(&total_count_query)
            .fetch_one(&state.db.pool)
            .await?
    };

    // Execute projects query
    let rows = if let Some(owner_address) = &params.owner_address {
        sqlx::query(&projects_query)
            .bind(owner_address)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await?
    } else {
        sqlx::query(&projects_query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await?
    };

    let projects: Vec<ProjectListItem> = rows
        .into_iter()
        .map(|row| {
            let tags: Vec<String> = row.get::<Vec<String>, _>("tags");
            
            ProjectListItem {
                id: row.get("id"),
                name: row.get("name"),
                owner_address: row.get("owner_address"),
                contract_address: row.get("contract_address"),
                description: row.get("description"),
                is_verified: row.get("is_verified"),
                verification_date: row.get("verification_date"),
                repository_url: row.get("repository_url"),
                bounty_amount: row.get("bounty_amount"),
                bounty_currency: row.get("bounty_currency"),
                bounty_expiry_date: row.get("bounty_expiry_date"),
                tags,
                created_at: row.get("created_at"),
                closed_at: row.get("closed_at"),
            }
        })
        .collect();

    let has_next = (offset + limit) < total_count;

    Ok(Json(ListProjectsResponse {
        projects,
        total_count,
        has_next,
    }))
}
