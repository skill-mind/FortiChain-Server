use axum::{Router, http::StatusCode, routing::get};

use crate::AppState;
use crate::db::Db;
pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/health_check", get(ping_handler))
}

#[tracing::instrument(name = "Health Check", skip(state))]
async fn ping_handler(state: axum::extract::State<AppState>) -> axum::http::StatusCode {
    match state.db.ping_db().await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
