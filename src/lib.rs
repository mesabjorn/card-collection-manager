pub mod api;
pub mod card;
pub mod cli;
pub mod copy;
pub mod db;
mod dberror; //custom db errors
pub mod jsoncards;
mod rarity;
pub mod series;

use std::sync::{Arc, Mutex};

use crate::db::DatabaseConnection;

pub struct AppState {
    pub db: Arc<Mutex<DatabaseConnection>>,
}
