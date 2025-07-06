use serde::Serialize;
use axum::{Json, http::StatusCode};


#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl From<ServiceError> for (StatusCode, Json<ErrorResponse>) {
    fn from(err: ServiceError) -> Self {
        let (status, error_type, message) = match err {
            ServiceError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_error",
                "Internal server error occurred",
            )
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