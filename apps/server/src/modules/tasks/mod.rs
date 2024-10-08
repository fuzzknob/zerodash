use crate::lunar::context::AppContext;
use axum::routing::get;
use axum::Router;

pub mod tasks_controller;

pub fn task_routes() -> Router<AppContext> {
    Router::new().route("/tasks", get(tasks_controller::index))
}
