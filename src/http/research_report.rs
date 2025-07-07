// src/http/research_report.rs
use axum::{extract::State, Json, http::StatusCode, response::IntoResponse, routing::post, Router};
use serde_json::json;
use uuid::Uuid;

use crate::{db::Db, models::{NewResearchReport, ResearchReport}, http::AppState};

// Handler for POST /reports
async fn create_report(
    State(app_state): State<AppState>,
    Json(payload): Json<NewResearchReport>,
) -> impl IntoResponse {
    match app_state.db.create_report(payload).await {
        Ok(report) => {
            // TODO: publish notification to validators
            // e.g., notification::notify_validators(&report).await;
            (StatusCode::CREATED, Json(report))
        }
        Err(err) => {
            eprintln!("DB error creating report: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Could not create report"})))
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new().route("/reports", post(create_report))
}
