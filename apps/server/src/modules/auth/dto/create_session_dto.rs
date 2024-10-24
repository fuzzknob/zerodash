use lunarus::prelude::{Datetime, Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionDTO {
    pub token: String,
    pub user: Id,
    pub expiration: Datetime,
}
