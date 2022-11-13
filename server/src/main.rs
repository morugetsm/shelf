mod controllers;
mod models;

use axum::{routing::get, Router, Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(controllers::user::get).delete(controllers::user::delete))
        .route("/user/:idx", get(controllers::user::get).delete(controllers::user::delete));

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "hi"
}
