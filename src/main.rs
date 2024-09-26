use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let server_address = "0.0.0.0:8000";
    let custom_tracing_subscriber = tracing_subscriber::fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(true)
        .finish();
    tracing::subscriber::set_global_default(custom_tracing_subscriber).unwrap();
    let server = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .expect("Couldn't bind listner");
    tracing::info!("Started server at {}", server_address);
    axum::serve(listener, server)
        .await
        .expect("There was an error while starting server");
}

async fn index() -> &'static str {
    "Zerodash Server"
}
