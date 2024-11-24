use lunarus::prelude::*;

pub async fn index() -> impl IntoResponse {
    res::message("Zerodash api")
}

#[debug_handler]
pub async fn up() -> impl IntoResponse {
    res::message("Okay")
}
