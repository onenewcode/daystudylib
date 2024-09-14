use axum::{
    routing::{get, post},
    Router,
};
use sse::{create_info, json_handler, path_handler2, sse_handler};
use stream_file::save_request_body;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod stream_file;
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
    .fallback_service(static_files_service)// 找不到对应的路由经行回调，找不到返回404
        .route("/sse", get(sse_handler))
        .route("/path2/:name/:age",get( path_handler2))
        .route("/json2", post(create_info))
        .route("/json", post(json_handler))
        .route("/file/:file_name", post(save_request_body))  // 添加文件上传文件例子
        // .route("/body", post(body_handler))

}
