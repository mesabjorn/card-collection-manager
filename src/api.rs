use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use tokio::task;

use crate::AppState;
use crate::card::Card;

#[derive(Debug, Serialize, Deserialize)]
struct Health {
    status: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/cards", get(list_cards))
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(Health { status: "ok" }))
}

async fn list_cards(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let db = state.db.clone();

    // spawn_blocking closure must return the data
    // spawn_blocking closure returns the vector
    let cards_with_meta: Vec<(Card, String, String)> = task::spawn_blocking(move || {
        let db = db.lock().unwrap(); // lock Mutex
        db.get_cards(None).unwrap() // call your method
    })
    .await
    .unwrap(); // unwrap the JoinHandle

    (StatusCode::OK, Json(cards_with_meta))
}
