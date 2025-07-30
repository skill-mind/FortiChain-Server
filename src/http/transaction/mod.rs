mod deposit;
mod domain;
mod withdraw;

use axum::{Router, routing::post};
pub use domain::*;

use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/deposit", post(deposit::deposit_handler))
        .route("/withdraw", post(withdraw::withdraw_handler))
}
