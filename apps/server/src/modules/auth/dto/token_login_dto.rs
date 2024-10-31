use lunarus::validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TokenLoginDto {
    #[garde(length(min = 5))]
    pub token: String,
}
