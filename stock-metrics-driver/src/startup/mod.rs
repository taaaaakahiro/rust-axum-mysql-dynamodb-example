use crate::{
    routes::{
        health::{hc},
    },
};
use axum::{
    routing::get,
    Router,
};
use dotenv::dotenv;
use std::{net::SocketAddr};

pub async fn startup() {
    let hc_router = Router::new()
        .route("/", get(hc));

    let app = Router::new()
        .nest("/hc", hc_router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"))
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
}
