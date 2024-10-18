use crate::{
    context::AppContext, database::initialize_database, errors::Error, prelude::run_migrations,
    res, utils::get_env, Result,
};
use axum::{extract::Request, http::StatusCode, Router, ServiceExt};
use tower_http::{normalize_path::NormalizePath, trace};

#[derive(Debug, Clone)]
pub struct LunarusApp {
    context: AppContext,
}

impl LunarusApp {
    pub async fn init() -> Result<Self> {
        initialize_environment()?;
        initialize_tracing()?;
        let db = initialize_database().await?;
        run_migrations(&db).await?;
        Ok(Self {
            context: AppContext { db },
        })
    }

    pub async fn start(self, router: Router<AppContext>) -> Result<()> {
        let server_address = get_env("SERVER_URL").unwrap_or("0.0.0.0".to_string());
        let server_port = get_env("SERVER_PORT").unwrap_or("8000".to_string());
        let server_full_url = format!("{server_address}:{server_port}");
        let listener = tokio::net::TcpListener::bind(&server_full_url)
            .await
            .map_err(|_| Error::TCPBindingError)?;
        let router = install_layers(router.with_state(self.context));
        let router = install_default_routes(router);
        let app = NormalizePath::trim_trailing_slash(router);
        tracing::info!("started server at {server_full_url}");
        axum::serve(listener, ServiceExt::<Request>::into_make_service(app)).await?;
        Ok(())
    }
}

fn initialize_environment() -> Result<()> {
    dotenvy::dotenv().map_err(|_| Error::EnvironmentInitializationError)?;
    Ok(())
}

fn initialize_tracing() -> Result<()> {
    let lunar_tracing = tracing_subscriber::fmt()
        .compact()
        .with_line_number(false)
        .finish();
    tracing::subscriber::set_global_default(lunar_tracing)
        .map_err(|_| Error::TracingInitializationError)
}

fn install_default_routes(router: Router) -> Router {
    router.fallback(|| async {
        res::builder()
            .status(StatusCode::NOT_FOUND)
            .message("The router you're looking for doesn't exists")
    })
}

fn install_layers(router: Router) -> Router {
    router.layer(
        trace::TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO))
            .on_failure(trace::DefaultOnFailure::new().level(tracing::Level::ERROR)),
    )
}
