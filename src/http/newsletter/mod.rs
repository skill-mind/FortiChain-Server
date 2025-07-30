pub mod domain;
pub mod subscribe;
mod verify_subscriber;

use crate::AppState;
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/newsletter", subscribe::router())
        .route(
            "/newsletter/verify",
            post(verify_subscriber::verify_subscriber),
        )
}
