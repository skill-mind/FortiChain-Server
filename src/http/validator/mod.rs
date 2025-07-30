
mod delete_profile;
mod domain;
mod view_profile;


use axum::{Router, routing::{delete, post}};
pub use domain::*;
pub use view_profile::*;

use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/validator/profile/delete",
            delete(delete_profile::delete_validator_profile),
        )
        .route(
            "/validator/profile/view",
            post(view_profile::view_validator_profile),
        )
}
