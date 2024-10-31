use super::Result;
use axum::{
    http,
    response::{Html, IntoResponse, Redirect, Response},
};
use axum_extra::extract::SignedCookieJar;
use serde::Serialize;
use serde_json::json;

pub fn text(content: &str) -> Result<Response> {
    Ok(content.to_string().into_response())
}

pub fn html(content: &str) -> Result<Response> {
    Ok(Html(content.to_string()).into_response())
}

pub fn json<T: Serialize>(content: T) -> Result<Response> {
    builder().json(content)
}

pub fn message(message: &str) -> Result<Response> {
    builder().message(message)
}

pub fn redirect(to: &str) -> Result<Response> {
    Ok(Redirect::to(to).into_response())
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

    pub fn signed_cookies(mut self, jar: SignedCookieJar) -> Self {
        let headers = self.response.headers_mut().unwrap();
        for cookie in jar.iter() {
            if let Ok(header_value) = cookie.encoded().to_string().parse() {
                headers.append("set-cookie", header_value);
            }
        }
        self
    }

    pub fn json<T: Serialize>(self, content: T) -> Result<Response> {
        let body = serde_json::to_string(&content).unwrap();
        Ok(self
            .response
            .header("content-type", "application/json")
            .body(body)?
            .into_response())
    }

    pub fn message(self, message: &str) -> Result<Response> {
        self.json(json!({ "message": message }))
    }
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}
