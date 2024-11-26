use std::sync::Arc;
use axum::{Router, extract::State};
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use tracing_subscriber::fmt::format;

mod toothless_php;
mod config;

async fn handle_request(
    State(state): State<AppState>,
    req: Request<axum::body::Body>,
) -> Response {
    let (parts, body) = req.into_parts();

    let method = parts.method.as_str();
    let uri = parts.uri;
    let path = uri.path();
    let query = uri.query().unwrap_or("");

    if path.contains("..") {
        return (axum::http::StatusCode::BAD_REQUEST, "Invalid Path").into_response();
    }

    let file_path = if path.ends_with('/') {
        format!("{}/index.php", state.root_dir)
    } else {
        format!("{}{}", state.root_dir, path)
    };

}

#[derive(Clone)]
struct AppState{}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState{};

    let app = Router::new()
        .route("/", get(root))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
