use lunarus::prelude::*;

pub async fn me() -> Result<impl IntoResponse> {
    res::message("todo profile")
}
