use axum::{Router, http::StatusCode, routing::get};

use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/health_check", get(health_check_handler))
}

async fn health_check_handler() -> StatusCode {
    tracing::info!("Health check endpoint called");
    StatusCode::OK
}
