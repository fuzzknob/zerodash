use lunarus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSpaceModel {
    pub id: Id,
    pub user_role: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl UserSpaceModel {
    pub const TABLE_NAME: &str = "users_spaces";
}
