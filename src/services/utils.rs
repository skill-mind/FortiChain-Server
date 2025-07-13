use axum::{Json, http::StatusCode};
use serde::Serialize;
use std::time::SystemTimeError;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("System Time Error: {0}")]
    SystemTimeError(#[from] SystemTimeError),
    #[error("Invalid Project ID")]
    InvalidProjectId(#[from] uuid::Error),
}

impl From<ServiceError> for (StatusCode, Json<ErrorResponse>) {
    fn from(err: ServiceError) -> Self {
        let (status, error_type, message) = match err {
            ServiceError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_error",
                "Internal server error occurred",
            ),
            ServiceError::SystemTimeError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "system_time_error",
                "Time went backwards",
            ),
            ServiceError::InvalidProjectId(_) => (
                StatusCode::BAD_REQUEST,
                "invalid_project_id",
                "The provided project ID is invalid",
            ),
        };

        (
            status,
            Json(ErrorResponse {
                error: error_type.to_string(),
                message: message.to_string(),
            }),
        )
    }
}
