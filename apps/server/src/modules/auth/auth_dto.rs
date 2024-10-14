use lunarus::validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterDTO {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 4))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginDTO {
    #[validate(length(min = 4))]
    pub identity: String,
    #[validate(length(min = 8))]
    pub password: String,
}
