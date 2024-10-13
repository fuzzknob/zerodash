use axum::{
    extract::rejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::collections::HashMap;
use validator::ValidationErrorsKind;

#[derive(thiserror::Error, Debug)]
pub enum Error {
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
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] rejection::JsonRejection),

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
            Error::ValidationError(validation_error) => {
                let mut errors = HashMap::<&str, Vec<String>>::new();
                for (key, kind) in validation_error.errors().to_owned() {
                    if let ValidationErrorsKind::Field(field_errors) = kind {
                        errors.insert(
                            key,
                            field_errors
                                .iter()
                                .map(|field_error| field_error.code.to_string())
                                .collect(),
                        );
                    }
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
