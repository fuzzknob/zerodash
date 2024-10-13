use lunarus::validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTaskDTO {
    #[validate(length(min = 1))]
    pub title: String,

    #[validate(length(min = 1))]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTaskDTO {
    #[validate(length(min = 1, max = 5))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub description: Option<String>,
}
