mod deposit;
mod domain;

use axum::{Router, routing::post};
pub use domain::*;

use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/deposit", post(deposit::deposit_handler))
}
