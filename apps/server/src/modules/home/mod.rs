use lunarus::prelude::*;

pub mod home_controller;

pub fn home_routes() -> Router<AppContext> {
    Router::new()
        .route("/", get(home_controller::index))
        .route("/up", get(home_controller::up))
}
