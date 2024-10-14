use lunarus::prelude::*;

pub mod auth_controller;
pub mod auth_dto;
pub mod auth_model;
pub mod auth_service;
mod session_dto;
mod session_model;

pub fn auth_routes() -> Router<AppContext> {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/register", post(auth_controller::register))
            .route("/login", post(auth_controller::login)),
    )
}
