use lunarus::prelude::*;

pub mod auth_controller;
pub mod auth_middleware;
pub mod auth_service;
pub mod dto;
pub mod model;

pub fn auth_routes() -> Router<AppContext> {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/register", post(auth_controller::register))
            .route("/login", post(auth_controller::login)),
    )
}
