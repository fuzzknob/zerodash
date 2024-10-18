use lunarus::prelude::*;
use zerodash_server::modules;

#[tokio::main]
async fn main() -> Result<()> {
    let app = LunarusApp::init().await?;
    let context = app.context.clone();
    app.start(modules::routes(context)).await
}
