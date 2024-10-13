use lunarus::prelude::*;
use serde::{Deserialize, Serialize};

pub const TASK_TABLE_NAME: &str = "tasks";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Id,
    pub title: String,
    pub description: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
