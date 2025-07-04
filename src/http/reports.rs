use axum::{
    Router,
    routing::get,
    extract::{State, Query, Path},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, FromRow};
use uuid::Uuid;

use crate::{AppState, error::Error};

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/api/projects/{project_id}/reports", get(list_reports))
}

#[derive(Debug, Deserialize)]
pub struct ListReportsQuery {
    status: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ReportResponse {
    id: String,
    title: String,
    status: String,
    severity: Option<String>,
    reported_by: String,
    validated_by: Option<String>,
    allocated_reward: Option<f64>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[axum::debug_handler]
#[tracing::instrument(name = "List Project Reports", skip(state))]
async fn list_reports(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Query(query): Query<ListReportsQuery>,
) -> Result<Json<Vec<ReportResponse>>, Error> {
    // First check if project exists
    let project_exists = match state.db.pool() {
        crate::db::DbPool::Postgres(pool) => {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM projects WHERE id = $1)")
                .bind(&project_id)
                .fetch_one(pool)
                .await
                .map_err(|_| Error::UnprocessableEntity {
                    errors: [(
                        "database".into(),
                        vec!["Failed to check project existence".into()],
                    )]
                    .into(),
                })?
        }
        crate::db::DbPool::Sqlite(pool) => {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM projects WHERE id = ?)")
                .bind(project_id.to_string())
                .fetch_one(pool)
                .await
                .map_err(|_| Error::UnprocessableEntity {
                    errors: [(
                        "database".into(),
                        vec!["Failed to check project existence".into()],
                    )]
                    .into(),
                })?
        }
    };

    if !project_exists {
        return Err(Error::NotFound);
    }

    // Build query for reports
    let (query_str, params) = match state.db.pool() {
        crate::db::DbPool::Postgres(_) => {
            let mut query_str = String::from(
                "SELECT id, title, status, severity, reported_by, validated_by, allocated_reward, created_at, updated_at 
                 FROM research_report 
                 WHERE project_id = $1",
            );
            let mut params = vec![project_id.to_string()];

            if let Some(status) = query.status {
                query_str.push_str(" AND status = $2");
                params.push(status);
            }

            query_str.push_str(" ORDER BY created_at DESC");
            (query_str, params)
        }
        crate::db::DbPool::Sqlite(_) => {
            let mut query_str = String::from(
                "SELECT id, title, status, severity, reported_by, validated_by, allocated_reward, created_at, updated_at 
                 FROM research_report 
                 WHERE project_id = ?",
            );
            let mut params = vec![project_id.to_string()];

            if let Some(status) = query.status {
                query_str.push_str(" AND status = ?");
                params.push(status);
            }

            query_str.push_str(" ORDER BY created_at DESC");
            (query_str, params)
        }
    };

    // Execute query based on database type
    let reports = match state.db.pool() {
        crate::db::DbPool::Postgres(pool) => {
            let mut query = sqlx::query_as::<_, ReportResponse>(&query_str);
            for param in &params {
                query = query.bind(param);
            }
            query.fetch_all(pool).await.map_err(|_| Error::UnprocessableEntity {
                errors: [(
                    "database".into(),
                    vec!["Failed to fetch reports".into()],
                )]
                .into(),
            })?
        }
        crate::db::DbPool::Sqlite(pool) => {
            let mut query = sqlx::query_as::<_, ReportResponse>(&query_str);
            for param in &params {
                query = query.bind(param);
            }
            query.fetch_all(pool).await.map_err(|_| Error::UnprocessableEntity {
                errors: [(
                    "database".into(),
                    vec!["Failed to fetch reports".into()],
                )]
                .into(),
            })?
        }
    };

    Ok(Json(reports))
} 