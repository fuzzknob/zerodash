use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct CreateSpaceDTO {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub primary: Option<bool>,
    pub user: String,
}
