mod close_project;
mod create_project;
mod domain;
mod project_detail_view;
mod shared;
mod verify_project;

use axum::{
    Router,
    routing::{get, post},
};
pub use domain::*;

use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/create_project",
            post(create_project::create_project_handler),
        )
        .route(
            "/closed_project",
            post(close_project::close_project_handler),
        )
        .route(
            "/projects/{project_id}/verify",
            post(verify_project::verify_project),
        )
        .route(
            "/projects/{project_id}",
            get(project_detail_view::get_project_detail_view),
        )
}
