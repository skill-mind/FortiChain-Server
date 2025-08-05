use axum::{extract::State, http::StatusCode, Json};
use super::types::AllocateBountyRequest;
use crate::AppState;

pub async fn allocate_bounty(
    State(state): State<AppState>,
    Json(payload): Json<AllocateBountyRequest>,
) -> Result<Json<String>, StatusCode> {
    // Placeholder implementation
    println!("Allocating bounty: {:?}", payload);
    Ok(Json("Bounty allocated".to_string()))
}