use axum::{response::Json, routing::get, Router};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
struct Message {
    text: String,
}

#[tokio::main]
async fn main() {
    // Permissive CORS layer so our frontend can fetch data locally
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any);

    // Build our application with a single API route
    let app = Router::new()
        .route("/api/hello", get(hello_handler))
        .layer(cors);

    // Run our app on http://127.0.0.1:4000
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Backend API listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler() -> Json<Message> {
    Json(Message {
        text: "Hello World from the Rust Backend!".to_string(),
    })
}
