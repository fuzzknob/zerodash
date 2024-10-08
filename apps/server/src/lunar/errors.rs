use crate::lunar::prelude::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum LunarError {
    #[error("Not Found")]
    NotFound,

    #[error("internal server error")]
    InternalServerError,

    #[error("tracing initialization error")]
    TracingInitializationError,

    #[error("environment initialization error")]
    EnvironmentInitializationError,

    #[error("database connection error")]
    DatabaseConnectionError,

    #[error("database initialization error")]
    DatabaseInitializationError,

    #[error("Database credential error")]
    DatabaseCredentialError,

    #[error("TCP binding error")]
    TCPBindingError,

    #[error("Axum error")]
    AxumError(#[from] axum::Error),

    #[error("Axum http error")]
    AxumHttpError(#[from] axum::http::Error),

    #[error("error while getting {0} environment variable")]
    EnvironmentVariableError(String),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error("std error")]
    IOError(#[from] std::io::Error),
    
    #[error(transparent)]
    DbError(#[from] surrealdb::Error),

    #[error(transparent)]
    Any(#[from] Box<dyn std::error::Error>),
}

impl IntoResponse for LunarError {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            LunarError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status_code, Json(json!({ "message": message }))).into_response()
    }
}
