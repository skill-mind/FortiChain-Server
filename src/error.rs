use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::http::header::WWW_AUTHENTICATE;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use sqlx::error::DatabaseError;
use std::borrow::Cow;
use std::collections::HashMap;
use std::time::SystemTimeError;
use thiserror::Error;

/// A common error type that can be used throughout the API.
#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid payload")]
    InvalidJsonBody(#[from] JsonRejection),

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("validation error: {0}")]
    ValidationError(#[from] garde::Report),

    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,

    /// Return `403 Forbidden`
    #[error("user may not perform that action")]
    Forbidden,

    /// Return `404 Not Found`
    #[error("request path not found")]
    NotFound,

    #[error("conflict")]
    Conflict,

    /// Return `422 Unprocessable Entity`
    #[error("error in the request body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    #[error("a database error has occurred: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("an internal server error has occurred")]
    InternalServerError(#[from] anyhow::Error),
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
            Self::InvalidJsonBody(_) | Self::InvalidRequest(_) | Self::ValidationError(_) => {
                StatusCode::BAD_REQUEST
            }
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Conflict => StatusCode::CONFLICT,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::DatabaseError(_) | Self::InternalServerError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("{}", self);

        match self {
            Self::InvalidJsonBody(_) | Self::InvalidRequest(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            Self::Forbidden => (
                StatusCode::FORBIDDEN,
                [(WWW_AUTHENTICATE, "Token")],
                self.to_string(),
            )
                .into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            Self::Conflict => (StatusCode::CONFLICT, self.to_string()).into_response(),
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
            Self::DatabaseError(_) | Self::InternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            Self::ValidationError(garde_errors) => {
                (StatusCode::BAD_REQUEST, garde_errors.to_string()).into_response()
            }
        }
    }
}

pub trait ResultExt<T> {
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::DatabaseError(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
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
    // decided to use variant more generic for other fields
    #[error("Amount cannot be zero or less")]
    InvalidAmount,
    #[error("Entity not found")]
    EntityNotFound,
    #[error("Insufficient funds")]
    InsufficientFunds,
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
            ServiceError::InvalidAmount => (
                StatusCode::BAD_REQUEST,
                "invalid_amount",
                "The provided amount is invalid",
            ),
            ServiceError::EntityNotFound => (
                StatusCode::NOT_FOUND,
                "entity_not_found",
                "The requested entity was not found",
            ),
            ServiceError::InsufficientFunds => (
                StatusCode::FORBIDDEN,
                "insufficient_funds",
                "Insufficient funds",
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
            ServiceError::InvalidAmount => {
                Error::InvalidRequest("Amount cannot be zero or less".to_string())
            }
            ServiceError::EntityNotFound => Error::NotFound,
            ServiceError::InsufficientFunds => Error::Forbidden,
        }
    }
}
