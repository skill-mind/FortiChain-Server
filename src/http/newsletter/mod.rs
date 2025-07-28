mod domain;
mod verify_subscriber;

use axum::{Router, routing::post};

use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new().route(
        "/newsletter/verify",
        post(verify_subscriber::verify_subscriber),
    )
}
