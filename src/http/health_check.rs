use axum::{Router, http::StatusCode, routing::get};

use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/health_check", get(|| async { StatusCode::OK }))
}
