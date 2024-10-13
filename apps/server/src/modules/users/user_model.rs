use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}
