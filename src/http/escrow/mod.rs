mod allocate_bounty;
mod domain;

use axum::{Router, routing::post};
pub use domain::*;

use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new().route(
        "/allocate_bounty",
        post(allocate_bounty::allocate_bounty_handler),
    )
}
