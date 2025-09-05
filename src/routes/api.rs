use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;
use crate::routes::cards; // bring in cards module
use crate::routes::series; // bring in series module

#[derive(Debug, Serialize, Deserialize)]
struct Health {
    status: &'static str,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .nest("/cards", cards::routes()) // mount cards under /cards
        .nest("/series", series::routes()) // mount series under /series
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(Health { status: "ok" }))
}
