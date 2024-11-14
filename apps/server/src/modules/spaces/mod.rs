use lunarus::prelude::*;
use middleware::from_fn_with_state;

use super::auth::auth_middleware;

pub mod dto;
pub mod model;
pub mod serializers;
pub mod space_controller;
pub mod space_service;

pub fn space_routes(context: AppContext) -> Router<AppContext> {
    Router::new()
        .route(
            "/spaces",
            get(space_controller::index).post(space_controller::create),
        )
        .route(
            "/spaces/:id",
            patch(space_controller::update).delete(space_controller::delete),
        )
        .layer(from_fn_with_state(
            context,
            auth_middleware::auth_middleware,
        ))
}
