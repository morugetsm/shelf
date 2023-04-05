mod controllers;
mod models;

use axum::{routing::get, Router, Server};
use std::net::SocketAddr;
// use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "hi"
}
