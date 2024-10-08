use serde_json::json;
use zerodash_server::lunar::prelude::*;
use zerodash_server::modules;

#[tokio::main]
async fn main() -> Result<()> {
    let app = LunarApp::init().await?;
    let router = Router::new()
        .route("/", get(index))
        .nest("/api", modules::routes());
    app.start(router).await
}

async fn index() -> impl IntoResponse {
    res::json(json!({
        "message": "It's working"
    }))
}
