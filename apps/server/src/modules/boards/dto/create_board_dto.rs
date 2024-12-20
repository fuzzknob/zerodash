use lunarus::validator::Validate;
use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateBoardDTO {
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(length(min = 1))]
    pub description: Option<String>,
    #[garde(length(min = 1))]
    pub icon: Option<String>,
    #[garde(length(min = 10))]
    pub space: String,
}
