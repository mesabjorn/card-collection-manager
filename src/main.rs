use std::{
    error::Error,
    io::{self, BufReader, Write},
};

mod card;
mod db;
mod rarity;

mod dberror; //custom db errors
mod jsoncards;
mod series;

use clap::Parser;

fn get_collection_number_from_string(s: &str) -> i32 {
    s.rsplit(|c: char| !c.is_ascii_digit()) // split from the end by non-digits
        .next() // take the first numeric chunk from the end
        .unwrap_or("0") // fallback if none
        .parse::<i32>() // parse as integer
        .unwrap_or(0)
}

use crate::{card::Card, db::DatabaseConnection, series::Series};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Database file name
    pub dbname: String,

    /// Mode of operation
    #[arg(short, long)]
    pub modus: String,

    /// JSON file with cards (required for modus=add_json)
    #[arg(short, long)]
    pub filename: Option<String>,
}

fn setup(dbname: &str) -> Result<DatabaseConnection, Box<dyn Error>> {
    let db = db::DatabaseConnection::new(dbname)?;
    db.create_tables()?;

    // Insert rarities
    db.insert_rarity("Common")?;
    db.insert_rarity("Rare")?;
    db.insert_rarity("Super Rare")?;
    db.insert_rarity("Ultra Rare")?;
    db.insert_rarity("Secret Rare")?;
    db.insert_rarity("Quarter Century Rare")?;

    Ok(db)
}

fn prompt_user_series() -> Result<Series, Box<dyn Error>> {
    let mut name = String::new();
    print!("Enter series name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    let mut release_year = String::new();
    print!("Enter release year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut release_year).unwrap();
    let release_year: i32 = release_year.trim().parse().unwrap_or(0);

    let mut n_cards = String::new();
    print!("Enter number of cards: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n_cards).unwrap();
    let n_cards: i32 = n_cards.trim().parse().unwrap_or(0);

    Ok(Series {
        id: None,
        name,
        release_year,
        n_cards,
    })
}

fn prompt_user_card() -> Result<Card, Box<dyn Error>> {
    let mut name = String::new();
    print!("Enter card name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    let name: String = name.trim().to_string();

    let mut card_number = String::new();
    print!("Enter card number (e.g LOB-EN000): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut card_number).unwrap();
    let number = card_number.trim().to_string();

    let mut series_id = String::new();
    print!("Enter series id: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut series_id).unwrap();
    let series_id: i32 = series_id.trim().parse().unwrap_or(0);

    let mut rarity_id = String::new();
    print!("Enter rarity id: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut rarity_id).unwrap();
    let rarity_id: i32 = rarity_id.trim().parse().unwrap_or(0);

    let mut collection_number = String::new();
    print!("Enter collection number (numeric): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut collection_number).unwrap();
    let collection_number: i32 = collection_number.trim().parse().unwrap_or(0);

    Ok(Card {
        name,
        series_id,
        number,
        collection_number: collection_number,
        in_collection: false,
        rarity_id,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse(); // Parse CLI arguments
    let dbname = args.dbname;

    let db = setup(&dbname)?;
    match args.modus.as_str() {
        "add_series" => {
            let series = prompt_user_series()?;
            let id = db.insert_series(&series)?;
            println!("Inserted series with ID {}", id);
        }
        "add_card" => {
            let card = prompt_user_card()?;
            let id = db.insert_card(&card)?;
            println!("Inserted card with ID {}", id);
        }
        "add_json" => {
            // Validate that filename is provided
            let filename = args
                .filename
                .as_ref()
                .ok_or_else(|| "--filename is required for add_json")?;

            let file = std::fs::File::open(filename)?;
            let reader = BufReader::new(file);

            let series_json: jsoncards::SeriesJson = serde_json::from_reader(reader)?;

            // Insert series
            let series = Series {
                id: None,
                name: series_json.name.clone(),
                release_year: series_json.release_year,
                n_cards: series_json.ncards,
            };

            let series_id = db.insert_series(&series)?;
            let mut cnt = 0;
            for c in series_json.cards {
                let card = Card {
                    name: c.name.clone(),
                    number: c.card_number.clone(),
                    collection_number: get_collection_number_from_string(&c.card_number),
                    rarity_id: db.get_rarity_id(&c.rarity)?, // directly i32
                    series_id: series_id,
                    in_collection: false,
                };
                let inserted_id = db.insert_card(&card)?;
                if inserted_id != 0 {
                    cnt += 1;
                }
            }
            println!("Inserted {} cards", cnt);
        }
        "list_cards" => {
            // Query cards
            for (card, rarity) in db.get_cards()? {
                println!("{:?} | Rarity: {}", card, rarity);
            }
        }
        _ => {
            println!("Unknown modus: {}", args.modus);
        }
    }

    Ok(())
}
