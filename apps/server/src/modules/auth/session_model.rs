use lunarus::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub const SESSION_TABLE_NAME: &str = "sessions";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionModel {
    #[serde(skip_serializing)]
    pub id: Id,
    pub token: String,
    pub expiration: Datetime,
    #[serde(skip_serializing)]
    pub user: Id,
    #[serde(skip_serializing)]
    pub created_at: Datetime,
    #[serde(skip_serializing)]
    pub updated_at: Datetime,
}

impl SessionModel {
    pub fn is_valid(&self) -> bool {
        self.expiration > Datetime::default()
    }
}
