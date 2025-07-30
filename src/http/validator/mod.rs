mod delete_profile;
mod domain;

use axum::{Router, routing::delete};
pub use domain::*;

use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new().route(
        "/validator/profile/delete",
        delete(delete_profile::delete_validator_profile),
    )
}
