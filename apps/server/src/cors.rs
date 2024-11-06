use axum::{
    http::header::{ACCEPT, CONTENT_TYPE},
    http::HeaderValue,
    Router,
};
use lunarus::{prelude::AppContext, utils::get_required_env};
use tower_http::cors::CorsLayer;

pub fn cors_plugin(_: AppContext, router: Router<AppContext>) -> Router<AppContext> {
    let web_frontend_url = get_required_env("WEB_URL");
    let homepage_url = get_required_env("HOMEPAGE_URL");
    let origins = [
        web_frontend_url.parse::<HeaderValue>().unwrap(),
        homepage_url.parse::<HeaderValue>().unwrap(),
    ];
    let cors_layer = CorsLayer::new()
        .allow_origin(origins)
        .allow_headers([CONTENT_TYPE, ACCEPT])
        .allow_credentials(true);
    router.layer(cors_layer)
}
