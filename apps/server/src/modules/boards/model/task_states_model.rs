use lunarus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatesModel {
    pub id: Id,
    pub color: String,
    pub index: i32,
    pub status_type: String,
    pub board: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl TaskStatesModel {
    pub const TABLE_NAME: &str = "task_states";
}
