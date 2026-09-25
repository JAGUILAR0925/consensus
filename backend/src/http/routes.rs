use std::sync::Arc;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use crate::application::elections::ElectionService;

pub fn router(service: Arc<ElectionService>) -> Router {
    Router::new().route("/health", get(health)).route("/api/v1/elections/demo", get(demo))
        .with_state(service).layer(CorsLayer::permissive()).layer(TraceLayer::new_for_http())
}
async fn health() -> (StatusCode, Json<serde_json::Value>) { (StatusCode::OK, Json(serde_json::json!({"status":"ok"}))) }
async fn demo(State(service): State<Arc<ElectionService>>) -> Json<crate::domain::Election> { Json(service.demo()) }
