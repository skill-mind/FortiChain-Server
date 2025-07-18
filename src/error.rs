use axum::Json;
use axum::http::StatusCode;
use axum::http::header::WWW_AUTHENTICATE;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::time::SystemTimeError;
use thiserror::Error;

/// A common error type that can be used throughout the API.
#[derive(Error, Debug)]
pub enum Error {
    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,

    /// Return `403 Forbidden`
    #[error("user may not perform that action")]
    Forbidden,

    /// Return `404 Not Found`
    #[error("request path not found")]
    NotFound,

    /// Return `422 Unprocessable Entity`
    #[error("error in the request body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },
}

impl Error {
    /// Convenient constructor for `Error::UnprocessableEntity`.
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();
        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }
        Self::UnprocessableEntity { errors: error_map }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::UnprocessableEntity { errors } => {
                #[derive(serde::Serialize)]
                struct Errors {
                    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
                }
                (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors { errors })).into_response()
            }
            Self::Unauthorized => (
                self.status_code(),
                [(WWW_AUTHENTICATE, "Token")],
                self.to_string(),
            )
                .into_response(),
            _ => (self.status_code(), self.to_string()).into_response(),
        }
    }
}

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

impl From<ServiceError> for Error {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::InvalidProjectId(_) => {
                Error::unprocessable_entity([("project_id", "is invalid")])
            }
            ServiceError::DatabaseError(_) => Error::Forbidden,
            ServiceError::SystemTimeError(_) => Error::Forbidden,
        }
    }
}
