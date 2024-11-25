use lunarus::prelude::*;
use middleware::from_fn_with_state;

use super::auth::auth_middleware;

pub mod board_controller;
pub mod board_service;
pub mod dto;
pub mod model;

pub fn board_routes(context: AppContext) -> Router<AppContext> {
    Router::new()
        .route("/boards", post(board_controller::create))
        .route(
            "/boards/:id",
            patch(board_controller::update).delete(board_controller::delete),
        )
        .layer(from_fn_with_state(
            context,
            auth_middleware::auth_middleware,
        ))
}
