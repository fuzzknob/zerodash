use lunarus::prelude::*;

pub mod user_controller;
pub mod user_model;

pub fn user_routes() -> Router<AppContext> {
    Router::new().route("/users/me", get(user_controller::me))
}
