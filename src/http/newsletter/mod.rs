pub mod domain;
pub mod subscribe;

use crate::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().nest("/newsletter", subscribe::router())
}
