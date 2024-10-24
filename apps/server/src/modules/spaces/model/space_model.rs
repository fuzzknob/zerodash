use lunarus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceModel {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub primary: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl SpaceModel {
    pub const TABLE_NAME: &str = "spaces";
}
