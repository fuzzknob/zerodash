use lunarus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardModel {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl BoardModel {
    pub const TABLE_NAME: &str = "boards";
}
