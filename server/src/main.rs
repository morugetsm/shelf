mod controllers;
mod models;

use axum::{routing::get, serve::serve, Router};
use chrono::Local;
use std::net::SocketAddr;
// use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("shelf-server v{}", env!("CARGO_PKG_VERSION"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let mut app = Router::new().route("/", get(root));
    {
        use controllers::user as con;
        app = app
            .route("/user", get(con::get).post(con::post))
            .route("/user/", get(con::get).post(con::post))
            .route(
                "/user/:id",
                get(con::get).patch(con::patch).delete(con::delete),
            );
    }
    // app = app.layer(CorsLayer::new().allow_origin(Any));

    tracing::info!("server addr: {}", addr);
    serve(listener, app).await.unwrap();
}

async fn root() -> String {
    format!("[{}] hi", Local::now().format("%Y-%m-%d %H:%M:%S"))
}
