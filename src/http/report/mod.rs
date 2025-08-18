mod domain;
mod reject_report;

use axum::{Router, routing::post};
pub use domain::*;

use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/report/reject", post(reject_report::reject_report))
}
