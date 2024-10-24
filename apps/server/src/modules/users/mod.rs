use lunarus::prelude::*;

pub mod model;
pub mod user_controller;

pub fn user_routes() -> Router<AppContext> {
    Router::new().route("/users/me", get(user_controller::me))
}
