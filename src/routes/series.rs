use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};

use std::sync::Arc;
use tokio::task;

use crate::AppState;
use crate::series::Series;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(list_series))
}

async fn list_series(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let db = state.db.clone(); // spawn_blocking closure must return the data // spawn_blocking closure returns the vector 
    let series: Vec<Series> = task::spawn_blocking(move || {
        let db = db.lock().unwrap(); // lock Mutex 
        db.get_unique_series().unwrap() // call your method
    })
    .await
    .unwrap(); // unwrap the JoinHandle 
    (StatusCode::OK, Json(series))
}
