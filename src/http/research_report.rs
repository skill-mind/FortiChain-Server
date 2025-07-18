use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use crate::{
    db::Db,
    http::{types::{NewReportRequest, ReportResponse}, AppState},
};
use uuid::Uuid;

async fn create_report(
    State(app_state): State<AppState>,
    Json(payload): Json<NewReportRequest>,
) -> impl IntoResponse {
    match app_state.db.create_report(&payload).await {
        Ok(report_model) => {
            // Fire-and-forget notification
            let _ = tokio::spawn(crate::notifications::notify_validators(
                report_model.id,
                report_model.project_id,
            ));

            let resp = ReportResponse {
                id: report_model.id,
                title: report_model.title,
                body: report_model.body,
                project_id: report_model.project_id,
                researcher_id: report_model.researcher_id,
                created_at: report_model.created_at.to_rfc3339(),
            };
            (StatusCode::CREATED, Json(resp))
        }
        Err(err) => {
            tracing::error!("Failed to create report: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Could not create report" })),
            )
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new().route("/reports", post(create_report))
}
