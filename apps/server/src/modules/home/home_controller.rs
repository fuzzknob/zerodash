use lunarus::prelude::*;

pub async fn index() -> impl IntoResponse {
    res::message("Zerodash api")
}

pub async fn up() -> impl IntoResponse {
    res::message("Okay")
}
