use lunarus::prelude::*;

pub mod task_dto;
pub mod task_model;
pub mod task_service;
pub mod tasks_controller;

pub fn task_routes() -> Router<AppContext> {
    Router::new()
        .route(
            "/tasks",
            get(tasks_controller::index).post(tasks_controller::create),
        )
        .route("/tasks/:id", patch(tasks_controller::update))
}
