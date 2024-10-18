use lunarus::prelude::*;
use middleware::from_fn_with_state;

use super::auth::auth_middleware;

pub mod task_dto;
pub mod task_model;
pub mod task_service;
pub mod tasks_controller;

pub fn task_routes(context: AppContext) -> Router<AppContext> {
    Router::new()
        .route(
            "/tasks",
            get(tasks_controller::index).post(tasks_controller::create),
        )
        .route("/tasks/:id", patch(tasks_controller::update))
        .layer(from_fn_with_state(
            context.clone(),
            auth_middleware::auth_middleware,
        ))
}
