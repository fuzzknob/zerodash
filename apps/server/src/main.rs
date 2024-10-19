use lunarus::prelude::*;
use zerodash_server::modules;

#[tokio::main]
async fn main() -> Result<()> {
    let app = LunarusApp::init().await?;
    app.start(modules::routes).await
}
