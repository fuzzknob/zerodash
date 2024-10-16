use super::task_dto::{CreateTaskDTO, UpdateTaskDTO};
use super::task_service::TaskService;
use lunarus::prelude::*;

#[debug_handler]
pub async fn index(State(db): State<Db>) -> Result<impl IntoResponse> {
    let tasks = TaskService::new(db).all().await?;
    res::json(tasks)
}

#[debug_handler]
pub async fn create(
    State(db): State<Db>,
    Json(new_task): Json<CreateTaskDTO>,
) -> Result<impl IntoResponse> {
    new_task.validate()?;
    TaskService::new(db).create_task(new_task).await?;
    res::builder()
        .status(StatusCode::CREATED)
        .message("Successfully created task")
}

#[debug_handler]
pub async fn update(
    Path(id): Path<String>,
    State(db): State<Db>,
    Json(task_updates): Json<UpdateTaskDTO>,
) -> Result<impl IntoResponse> {
    TaskService::new(db).update_task(id, task_updates).await?;
    res::message("Successfully updated task")
}
