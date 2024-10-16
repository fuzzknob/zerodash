use lunarus::validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTaskDTO {
    #[garde(required, length(min = 10), email)]
    pub title: Option<String>,

    #[garde(required, length(min = 1))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskDTO {
    // #[validate(length(min = 1, max = 5))]
    pub title: Option<String>,
    // #[validate(length(min = 1))]
    pub description: Option<String>,
}

pub struct HelloStruct {
    pub title: Option<String>,
}
