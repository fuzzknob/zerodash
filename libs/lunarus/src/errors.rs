use axum::{
    extract::rejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Not Found")]
    NotFound,

    #[error("Unauthenticated")]
    Unauthenticated,

    #[error("Unauthorized")]
    Unauthorized,

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

    #[error("Database query error")]
    DatabaseQueryError,

    #[error("TCP binding error")]
    TCPBindingError,

    #[error("Axum error")]
    AxumError(#[from] axum::Error),

    #[error("Axum http error")]
    AxumHttpError(#[from] axum::http::Error),

    #[error("error while getting {0} environment variable")]
    EnvironmentVariableError(String),

    #[error("{0}")]
    EmailSendError(String),

    // Third party errors
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error("std error")]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    DbError(#[from] surrealdb::Error),

    #[error(transparent)]
    ValidationError(#[from] garde::Report),

    #[error(transparent)]
    AxumFormRejection(#[from] rejection::JsonRejection),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    RenderError(#[from] handlebars::RenderError),

    #[error(transparent)]
    Any(#[from] Box<dyn std::error::Error>),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let response = match self {
            Error::NotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({ "message": self.to_string() })),
            ),
            Error::Unauthenticated => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": self.to_string() })),
            ),
            Error::Unauthorized => (
                StatusCode::FORBIDDEN,
                Json(json!({ "message": self.to_string() })),
            ),
            Error::ValidationError(validation_error) => {
                let mut errors = HashMap::<String, Vec<String>>::new();
                for (path, error) in validation_error.iter() {
                    let key = path.to_string();
                    let message = error.message();
                    if let Some(error) = errors.get_mut(&key) {
                        error.push(message.to_string());
                        continue;
                    }
                    errors.insert(key, vec![message.to_string()]);
                }
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "There was a validation error",
                        "errors": errors
                    })),
                )
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": self.to_string() })),
            ),
        };
        response.into_response()
    }
}
