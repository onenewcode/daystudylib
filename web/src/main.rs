use axum::{
    middleware::{self},
    routing::{get, post},
    Router,
};
use sse::{ sse_handler};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod sse;
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application
    let app = app();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    let assets_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);
    // build our application with a route
    Router::new()
        .route("/sse", get(sse_handler))
        // .route("/body", post(body_handler))
    
}
