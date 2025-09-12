pub mod card;
pub mod cardtype;
pub mod cli;
pub mod copy;
pub mod db;
mod dberror; //custom db errors
pub mod jsoncards;
pub mod rarity;
pub mod series;

pub mod routes;

use std::sync::{Arc, Mutex};

use crate::db::DatabaseConnection;

pub struct AppState {
    pub db: Arc<Mutex<DatabaseConnection>>,
}
