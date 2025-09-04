use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::task;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCard {
    pub name: String,
    pub series_id: i32,
    pub number: String,
    pub collection_number: i32,
    pub in_collection: i32,
    pub rarity_id: i32,
    pub card_type_id: i32,
}

use crate::AppState;
use crate::card::Card;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(list_cards).post(search_cards))
}

async fn list_cards(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let db = state.db.clone(); // spawn_blocking closure must return the data // spawn_blocking closure returns the vector 
    let cards_with_meta: Vec<(Card, String, String)> = task::spawn_blocking(move || {
        let db = db.lock().unwrap(); // lock Mutex 
        db.get_cards(None).unwrap() // call your method
    })
    .await
    .unwrap(); // unwrap the JoinHandle 
    (StatusCode::OK, Json(cards_with_meta))
}

#[derive(Debug, Deserialize)]
struct SearchRequest {
    name: Option<String>,
}
async fn search_cards(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchRequest>,
) -> impl IntoResponse {
    let query = match payload.name {
        Some(ref q) if !q.is_empty() => q.clone(),
        _ => return (StatusCode::FORBIDDEN, "name is required").into_response(),
    };

    let db = state.db.clone();

    let results: Vec<(Card, String, String)> = task::spawn_blocking(move || {
        let db: std::sync::MutexGuard<'_, crate::db::DatabaseConnection> = db.lock().unwrap();
        db.get_cards(Some(&query)).unwrap()
    })
    .await
    .unwrap();

    (StatusCode::OK, Json(results)).into_response()
}

// async fn get_card(State(state): State<Arc<AppState>>, Path(id): Path<i32>) -> impl IntoResponse {
//     let db = state.db.clone();

//     let card: Option<Card> = task::spawn_blocking(move || {
//         let db = db.lock().unwrap();
//         db.get_card(id).ok()
//     })
//     .await
//     .unwrap();

//     match card {
//         Some(c) => (StatusCode::OK, Json(c)).into_response(),
//         None => (StatusCode::NOT_FOUND, "Card not found").into_response(),
//     }
// }
