use lunarus::prelude::*;

use crate::utils::hash::get_unique_random_hash;

pub async fn index() -> impl IntoResponse {
    res::message("Zerodash api")
}

#[debug_handler]
pub async fn up() -> impl IntoResponse {
    let random = get_unique_random_hash();
    res::message(&random)
    // res::message("Okay")
}
