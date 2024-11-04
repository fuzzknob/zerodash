use lunarus::prelude::*;
use zerodash_server::{cors::cors_plugin, modules};

#[tokio::main]
async fn main() -> Result<()> {
    let app = LunarusApp::init().await?;
    app.plug(modules::routes)
        .plug(default_plugins)
        .plug(cors_plugin)
        .start()
        .await
}
