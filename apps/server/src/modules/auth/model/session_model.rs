use lunarus::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub const TABLE_NAME: &str = "sessions";

    pub fn is_valid(&self) -> bool {
        self.expiration > Datetime::default()
    }
}
