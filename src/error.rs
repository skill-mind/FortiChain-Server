use axum::{http::{StatusCode, HeaderMap, header}, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Unprocessable Entity")] 
    UnprocessableEntity(Errors),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Serialize, Debug, Default)]
pub struct Errors {
    pub errors: HashMap<String, Vec<String>>,
}

impl Errors {
    pub fn new() -> Self {
        Self { errors: HashMap::new() }
    }
    pub fn add(mut self, field: impl Into<String>, message: impl Into<String>) -> Self {
        self.errors.entry(field.into()).or_default().push(message.into());
        self
    }
}

impl Error {
    pub fn unprocessable_entity(errors: HashMap<String, Vec<String>>) -> Self {
        Error::UnprocessableEntity(Errors { errors })
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::Sqlx(_) | Error::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = self.status_code();
        match self {
            Error::UnprocessableEntity(errors) => {
                (status, Json(errors)).into_response()
            }
            Error::Unauthorized => {
                let mut headers = HeaderMap::new();
                headers.insert(header::WWW_AUTHENTICATE, "Bearer".parse().unwrap());
                (status, headers, self.to_string()).into_response()
            }
            Error::Sqlx(ref err) => {
                log::error!("SQLx error: {err:?}");
                (status, "Internal server error").into_response()
            }
            Error::Anyhow(ref err) => {
                log::error!("Anyhow error: {err:?}");
                (status, "Internal server error").into_response()
            }
            _ => (status, self.to_string()).into_response(),
        }
    }
}

pub trait ResultExt<T> {
    fn on_constraint<F>(self, constraint: &str, f: F) -> Self
    where
        F: FnOnce(&sqlx::Error) -> Error;
}

impl<T> ResultExt<T> for Result<T, Error> {
    fn on_constraint<F>(self, constraint: &str, f: F) -> Self
    where
        F: FnOnce(&sqlx::Error) -> Error,
    {
        self.map_err(|e| {
            if let Error::Sqlx(ref sqlx_err) = e {
                if let sqlx::Error::Database(db_err) = sqlx_err {
                    if let Some(db_constraint) = db_err.constraint() {
                        if db_constraint == constraint {
                            return f(sqlx_err);
                        }
                    }
                }
            }
            e
        })
    }
}
