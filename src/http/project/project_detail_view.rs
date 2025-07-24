use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    AppState, Error,
    http::project::{ProjectResponse, shared::get_project_by_id},
};

#[tracing::instrument(name = "Get Project", skip(state), fields(project_id = %project_id))]
pub async fn get_project_detail_view(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ProjectResponse>, Error> {
    let project = get_project_by_id(&state.db.pool, project_id).await?;
    Ok(Json(project))
}
