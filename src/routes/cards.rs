use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
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

use crate::rarity::Rarity;
use crate::{AppState, cardtype::CardType, dberror::DbError, series::Series};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route(
        "/",
        get(list_cards).post(search_cards).put(update_card_count),
    )
}

#[derive(Serialize)]
struct CardWithMeta {
    number: String,
    name: String,
    series: Series,
    in_collection: i32,
    cardtype: CardType,
    cardtype_display: String,
    rarity: Rarity,
}

pub async fn list_cards(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let db = state.db.clone();

    let cards_with_meta: Vec<CardWithMeta> = task::spawn_blocking(move || {
        let db = db.lock().unwrap();

        // Assuming your DB returns Vec<(Card, String, String)>
        db.get_cards(None)
            .unwrap()
            .into_iter()
            .map(|card| CardWithMeta {
                number: card.number,
                name: card.name,
                series: card.series,
                in_collection: card.in_collection,
                rarity: card.rarity,
                cardtype: card.cardtype.clone(),
                cardtype_display: card.cardtype.display(),
            })
            .collect()
    })
    .await
    .unwrap();

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

    let cards_with_meta: Vec<CardWithMeta> = task::spawn_blocking(move || {
        let db: std::sync::MutexGuard<'_, crate::db::DatabaseConnection> = db.lock().unwrap();
        // Assuming your DB returns Vec<(Card, String, String)>
        db.get_cards(Some(&query))
            .unwrap()
            .into_iter()
            .map(|card| CardWithMeta {
                number: card.number,
                name: card.name,
                series: card.series,
                in_collection: card.in_collection,
                rarity: card.rarity,
                cardtype: card.cardtype.clone(),
                cardtype_display: card.cardtype.display(),
            })
            .collect()
    })
    .await
    .unwrap();

    (StatusCode::OK, Json(cards_with_meta)).into_response()
}

#[derive(Debug, Deserialize)]
pub struct UpdateCardRequest {
    pub id: String,
    #[serde(default)] // optional, defaults to None if missing
    pub number: Option<i32>, //number in collection, defaults to add 1
}

async fn update_card_count(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateCardRequest>,
) -> impl IntoResponse {
    let db = state.db.clone();

    //default to add one
    let number = payload.number; // Option<i32>

    let id = payload.id;

    let result: Result<i32, DbError> = task::spawn_blocking(move || {
        let db = db.lock().unwrap();

        match number {
            Some(-1) => {
                // Selling card
                db.sell_card(&id, 1).map(|_| -1) // return -1 or any meaningful marker
            }
            other => {
                // Collecting card, default Some(1)
                //let count = other.or(Some(1));
                db.collect_card(&id, other)
            }
        }
    })
    .await
    .unwrap();

    match result {
        Ok(card) => (StatusCode::OK, Json(card)).into_response(),
        Err(e) => {
            // map DB errors into proper HTTP codes
            let status = match e {
                DbError::InvalidOperation(_) => StatusCode::BAD_REQUEST, // catch all InvalidOperation
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, Json(format!("Database error: {}", e))).into_response()
        }
    }
}
