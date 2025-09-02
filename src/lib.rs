pub mod api;
pub mod card;
mod cli;
mod copy;
pub mod db;
mod dberror; //custom db errors
mod jsoncards;
mod rarity;
mod series;

use std::sync::{Arc, Mutex};

use crate::db::DatabaseConnection;

pub struct AppState {
    pub db: Arc<Mutex<DatabaseConnection>>,
}
