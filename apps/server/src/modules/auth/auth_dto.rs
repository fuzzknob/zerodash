use lunarus::validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterDTO {
    #[garde(required, length(min = 1))]
    pub name: Option<String>,
    #[garde(required, length(min = 4))]
    pub username: Option<String>,
    #[garde(required, email, length(min = 4))]
    pub email: Option<String>,
    #[garde(required, length(min = 8))]
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginDTO {
    #[garde(required, length(min = 4))]
    pub identity: Option<String>,
    #[garde(required, length(min = 8))]
    pub password: Option<String>,
}
