use lunarus::validator::Validate;
use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateSpaceDTO {
    #[garde(length(min = 1))]
    pub name: Option<String>,
    #[garde(length(min = 5))]
    pub slug: Option<String>,
    #[garde(length(min = 1))]
    pub description: Option<String>,
    #[garde(length(min = 5))]
    pub icon: Option<String>,
}
