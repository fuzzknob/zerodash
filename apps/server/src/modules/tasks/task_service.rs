use super::task_dto::{CreateTaskDTO, UpdateTaskDTO};
use super::task_model::{Task, TASK_TABLE_NAME};
use lunarus::prelude::*;

pub struct TaskService {
    db: Db,
}

impl TaskService {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn all(&self) -> Result<Vec<Task>> {
        Ok(self.db.select(TASK_TABLE_NAME).await?)
    }

    pub async fn create_task(self, new_task: CreateTaskDTO) -> Result<Option<Task>> {
        let task: Option<Task> = self.db.create(TASK_TABLE_NAME).content(new_task).await?;
        Ok(task)
    }

    pub async fn update_task(
        self,
        id: String,
        task_updates: UpdateTaskDTO,
    ) -> Result<Option<Task>> {
        let task: Option<Task> = self
            .db
            .update((TASK_TABLE_NAME, id))
            .merge(task_updates)
            .await?;
        Ok(task)
    }

    // pub async fn update_task(
    //     self,
    //     id: String,
    //     task_updates: UpdateTaskDTO,
    // ) -> Result<Option<Task>> {
    //     let task: Option<Task> = self
    //         .db
    //         .update((TASK_TABLE_NAME, id))
    //         .merge(task_updates)
    //         .await?;
    //     Ok(task)
    // }
}
