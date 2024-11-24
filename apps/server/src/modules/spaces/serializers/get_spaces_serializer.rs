use lunarus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::modules::boards::model::BoardModel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpaceSerializer {
    pub id: Id,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub primary: bool,
    pub boards: Vec<BoardModel>,
}
