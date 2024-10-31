use lunarus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeTokenModel {
    pub id: Id,
    pub token: String,
    pub expiration: Datetime,
    pub user: Id,
    #[serde(skip_serializing)]
    pub created_at: Datetime,
    #[serde(skip_serializing)]
    pub updated_at: Datetime,
}

impl ExchangeTokenModel {
    pub const TABLE_NAME: &str = "exchange_token";

    pub fn is_valid(&self) -> bool {
        self.expiration > Datetime::default()
    }
}
