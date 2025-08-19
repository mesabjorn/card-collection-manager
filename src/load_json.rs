use card::Card;
mod db;
mod series;

use crate::db::DatabaseConnection;
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CardJson {
    pub card_number: String,
    pub name: String,
    pub rarity: String,
    pub category: String,
}

fn main() -> rusqlite::Result<()> {
    let args = Args::parse(); // Parse CLI arguments
    let json_path = args.file;

    let db = DatabaseConnection::new("cards.db")?;

    // Open and read JSON file
    let file = File::open(&json_path).expect("Cannot open JSON file");
    let reader = BufReader::new(file);
    let cards: Vec<CardJson> = from_reader(reader).expect("Cannot parse JSON");

    for card_json in cards {
        // Insert or get rarity ID
        let rarity_id = db.insert_rarity(&card_json.rarity)?;

        // Insert or get series ID (here category as series for example)
        let series_id = db.insert_series(&series::Series {
            id: None,
            name: card_json.category.clone(),
            release_year: 0, // default/static
            n_cards: 0,
        })?;

        // Insert card
        let card = card::Card {
            name: card_json.name.clone(),
            number: card_json.card_number.clone(),
            in_collection: false,
            rarity_id,
            series_id,
        };

        db.insert_card(&card)?;
    }

    println!("Inserted {} cards from '{}'", cards.len(), json_path);
    Ok(())
}
