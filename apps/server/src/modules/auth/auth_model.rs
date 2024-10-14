use lunarus::prelude::*;
use serde::{Deserialize, Serialize};

pub const AUTH_TABLE_NAME: &str = "users";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthModel {
    pub id: Id,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
