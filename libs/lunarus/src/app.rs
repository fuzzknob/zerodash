use crate::{
    context::AppContext,
    database::initialize_database,
    errors::Error,
    prelude::run_migrations,
    res,
    utils::{get_env, get_required_env},
    Result,
};
use axum::{extract::Request, http::StatusCode, Router, ServiceExt};
use axum_extra::extract::cookie::Key;
use tower_http::{normalize_path::NormalizePath, trace};

#[derive(Debug, Clone)]
pub struct LunarusApp {
    pub context: AppContext,
    pub router: Router<AppContext>,
}

impl LunarusApp {
    pub async fn init() -> Result<Self> {
        initialize_environment()?;
        initialize_tracing()?;
        let db = initialize_database().await?;
        let key = get_required_env("COOKIE_KEY");
        let key = Key::from(key.as_bytes());
        run_migrations(&db).await?;
        Ok(Self {
            context: AppContext { db, key },
            router: Router::new(),
        })
    }

    pub fn plug(
        mut self,
        plugin: fn(AppContext, Router<AppContext>) -> Router<AppContext>,
    ) -> Self {
        let router = plugin(self.context.clone(), self.router);
        self.router = router;
        self
    }

    pub async fn start(self) -> Result<()> {
        let server_address = get_env("SERVER_URL").unwrap_or("0.0.0.0".to_string());
        let server_port = get_env("SERVER_PORT").unwrap_or("8000".to_string());
        let server_full_url = format!("{server_address}:{server_port}");
        let listener = tokio::net::TcpListener::bind(&server_full_url)
            .await
            .map_err(|_| Error::TCPBindingError)?;
        let router = self.router.with_state(self.context);
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

pub fn default_plugins(_: AppContext, router: Router<AppContext>) -> Router<AppContext> {
    router
        .fallback(|| async {
            res::builder()
                .status(StatusCode::NOT_FOUND)
                .message("The router you're looking for doesn't exists")
        })
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(tracing::Level::ERROR)),
        )
}
