mod adapters;
mod application;
mod domain;
mod http;

use std::sync::Arc;
use axum::Router;
use application::elections::ElectionService;
use http::routes::router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let service = Arc::new(ElectionService::default());
    let app: Router = router(service);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Consensus API listening on 0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
