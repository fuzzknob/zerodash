use super::Result;
use axum::{
    http,
    response::{Html, IntoResponse, Redirect},
    Json,
};
use bytes::{BufMut, BytesMut};
use serde::Serialize;
use serde_json::json;

pub fn text(content: &str) -> Result<impl IntoResponse> {
    Ok(content.to_string())
}

pub fn html(content: &str) -> Result<impl IntoResponse> {
    Ok(Html(content.to_string()))
}

pub fn json<T: Serialize>(content: T) -> Result<impl IntoResponse> {
    Ok(Json(content))
}

pub fn message(message: &str) -> Result<impl IntoResponse> {
    builder().message(message)
}

pub fn redirect(to: &str) -> Result<impl IntoResponse> {
    Ok(Redirect::to(to))
}

pub fn builder() -> ResponseBuilder {
    ResponseBuilder::new()
}

pub struct ResponseBuilder {
    response: http::response::Builder,
}

impl ResponseBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            response: http::Response::builder(),
        }
    }

    pub fn status(self, status: http::StatusCode) -> Self {
        Self {
            response: self.response.status(status),
        }
    }

    pub fn header(self, key: &str, value: &str) -> Self {
        Self {
            response: self.response.header(key, value),
        }
    }

    pub fn json<T: Serialize>(self, content: T) -> Result<impl IntoResponse> {
        let mut buf = BytesMut::with_capacity(128).writer();
        serde_json::to_writer(&mut buf, &content)?;
        let body = axum::body::Body::from(buf.into_inner().freeze());
        Ok(self
            .response
            .header("content-type", "application/json")
            .body(body)?)
    }

    pub fn message(self, message: &str) -> Result<impl IntoResponse> {
        self.json(json!({ "message": message }))
    }
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}
