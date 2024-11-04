use axum::{http::HeaderValue, Router};
use lunarus::{prelude::AppContext, utils::get_required_env};
use tower_http::cors::{Any, CorsLayer};

pub fn cors_plugin(_: AppContext, router: Router<AppContext>) -> Router<AppContext> {
    let web_frontend_url = get_required_env("WEB_URL");
    let cors_layer = CorsLayer::new()
        .allow_origin(web_frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods(Any);
    router.layer(cors_layer)
}
