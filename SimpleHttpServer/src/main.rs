use axum::{
    extract::Json,
    http::StatusCode,
    response::Response,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Deserialize, Serialize)] // <-- add Serialize here
struct Input {
    name: String,
    email:String,
    msg:String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    message: String,
    data: Option<T>,
}

// POST handler
async fn handle_post(Json(payload): Json<Input>) -> Response {
    let response = ApiResponse {
        status: "success".to_string(),
        message: format!("Hello, {}!", payload.email),
        data: Some(payload),
    };

    // Serialize to JSON string
    let body = serde_json::to_string(&response).unwrap();

    // Build Response manually
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/post", post(handle_post));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
