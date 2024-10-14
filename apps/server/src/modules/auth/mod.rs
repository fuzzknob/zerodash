use lunarus::prelude::*;

pub mod auth_controller;
pub mod auth_dto;
pub mod auth_model;
pub mod auth_service;

pub fn auth_routes() -> Router<AppContext> {
    Router::new().nest(
        "/auth",
        Router::new().route("/register", post(auth_controller::register)),
    )
}
