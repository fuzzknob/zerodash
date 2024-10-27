use lunarus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserModel {
    pub id: Id,
    pub name: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email_verified_at: Option<Datetime>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl UserModel {
    pub const TABLE_NAME: &str = "users";

    pub fn first_name(&self) -> String {
        let split: Vec<&str> = self.name.split_whitespace().collect();
        split.get(0).unwrap().to_string()
    }
}
