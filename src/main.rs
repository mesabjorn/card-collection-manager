use std::{
    error::Error,
    io::{self, BufReader, Write},
};

mod card;
mod cli;
mod copy;
mod db;
mod dberror; //custom db errors
mod jsoncards;
mod rarity;
mod series;

use clap::Parser;
use open;

use crate::{
    card::Card,
    cli::{Args, Command},
    copy::add_file_to_clipboard,
    db::DatabaseConnection,
    series::Series,
};

pub fn get_series_and_number(s: &str) -> (String, i32) {
    // Find the position where the numeric part starts from the end
    let pos = s
        .rfind(|c: char| c.is_ascii_digit())
        .map(|last_digit_idx| {
            // Find the start of the contiguous digit block
            s[..=last_digit_idx]
                .rfind(|c: char| !c.is_ascii_digit())
                .map_or(0, |idx| idx + 1)
        })
        .unwrap_or(s.len());

    let (prefix, num_str) = s.split_at(pos);
    let number = num_str.parse::<i32>().unwrap_or(0);

    // Take only the part before the first hyphen
    let abbr = prefix.split('-').next().unwrap_or("").to_string();

    (abbr, number)
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

    let mut release_date = String::new();
    print!("Enter release date (%Y-%m-%d): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut release_date).unwrap();
    let release_date = release_date.trim().to_string();

    let mut n_cards = String::new();
    print!("Enter number of cards: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n_cards).unwrap();
    let n_cards: i32 = n_cards.trim().parse().unwrap_or(0);

    Ok(Series {
        id: None,
        name,
        release_date,
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
        in_collection: 0,
        rarity_id,
    })
}

fn format_card(card: &Card, rarity: &str, series: &str, formatter: &str) -> String {
    formatter
        .replace("{name}", &card.name)
        .replace("{number}", &card.number)
        .replace("{collection_number}", &card.collection_number.to_string())
        .replace("{rarity}", rarity)
        .replace("{series}", series)
        .replace("{in_collection}", &card.in_collection.to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse(); // Parse CLI arguments
    let dbname = args.dbname;

    let db = setup(&dbname)?;
    match args.command {
        Command::Init {} => {
            println!("Initialized tables in database")
        }
        Command::Add {
            kind,
            name,
            filename,
        } => {
            match kind.as_str() {
                "series" => {
                    let series = prompt_user_series()?;
                    let id = db.insert_series(&series)?;
                    println!("Inserted series with ID {}", id);
                }
                "card" => {
                    let card = prompt_user_card()?;
                    let id = db.insert_card(&card)?;
                    println!("Inserted card with ID {}", id);
                }
                "json" => {
                    // Validate that filename is provided
                    let filename = filename.expect("--filename is required for add json");

                    let file = std::fs::File::open(filename)?;
                    let reader = BufReader::new(file);

                    let series_json: jsoncards::SeriesJson = serde_json::from_reader(reader)?;

                    // Insert series
                    let series = Series {
                        id: None,
                        name: series_json.name.clone(),
                        release_date: series_json.release_date,
                        n_cards: series_json.ncards,
                    };

                    let series_id = db.insert_series(&series)?;
                    let mut cnt = 0;
                    for c in series_json.cards {
                        let (abbr, collection_number) = get_series_and_number(&c.card_number);
                        let card = Card {
                            name: c.name.clone(),
                            number: format!("{}-{:03}", abbr, collection_number),
                            collection_number: collection_number,
                            rarity_id: db.get_rarity_id(&c.rarity)?, // directly i32
                            series_id: series_id,
                            in_collection: 0,
                        };
                        let inserted_id = db.insert_card(&card)?;
                        if inserted_id != 0 {
                            cnt += 1;
                        }
                    }
                    println!("Inserted {} cards", cnt);
                }
                "rarity" => {
                    let n = name.expect("--name for rarity is required");
                    db.insert_rarity(&n)?;
                    println!("Inserted rarity '{}'", n);
                }
                _ => {
                    println!("Unknown kind: {}", kind);
                }
            }
        }
        Command::List {
            kind,
            name,
            formatter,
            hide_collected,
        } => {
            match kind.as_str() {
                "cards" => {
                    let cards = db.get_cards(None)?;
                    for (card, rarity) in cards {
                        println!("{:?} | Rarity: {}", card, rarity);
                    }
                }
                "serie" => {
                    let series_name = name.expect("--name is required for list series");

                    // Query cards
                    let cards = db.get_cards_by_seriesname(&series_name)?;
                    for (card, rarity, series) in cards {
                        if hide_collected && card.in_collection > 0 {
                            continue;
                        }
                        println!("{}", format_card(&card, &rarity, &series, &formatter));
                    }
                }
                "series" => {
                    // list current unique series in db
                    let series_list = db.get_unique_series()?;
                    let mut cnt = 1;
                    if series_list.len() == 0 {
                        println!("No series in current database");
                    }
                    for s in series_list {
                        println!(
                            "{}. {} ({}) - {} cards",
                            cnt, s.name, s.release_date, s.n_cards
                        );
                        cnt += 1;
                    }
                }
                _ => {
                    println!("Unknown kind: {}", kind);
                }
            }
        }
        Command::Collect { id, count } => {
            //for collecting card id's (e.g. PSV-EN001)
            if id.len() == 1 {
                let card_id = &id[0];

                let new_count = db.collect_card(&card_id, count)?;

                println!(
                    "Card '{}' now has {} copies in collection.",
                    card_id, new_count
                );
            } else {
                for card_id in id {
                    let new_count = db.collect_card(&card_id, count)?;
                    println!(
                        "Card {} now has {} copies in collection.",
                        card_id, new_count
                    );
                }
            }
        }
        Command::Find { kind, query } => {
            match kind.as_str() {
                "cards" => {
                    let cards = db.get_cards(query.as_deref())?;
                    for (card, rarity) in cards {
                        println!("{:?} | Rarity: {}", card, rarity);
                    }
                }
                "serie" => {
                    let q = query.expect("--query is required for find serie");
                    let skip = ["the", "of"];
                    let result: String = q
                        .split_whitespace() // split into words
                        .map(|word| {
                            if skip.contains(&word) {
                                word.to_string() // keep as-is (lowercase)
                            } else {
                                let mut chars = word.chars();
                                match chars.next() {
                                    Some(first) => {
                                        first.to_uppercase().collect::<String>() + chars.as_str()
                                    }
                                    None => String::new(),
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("_");
                    let url = format!("https://yugioh.fandom.com/wiki/{}", result);
                    open::that(url)?;

                    add_file_to_clipboard("./cardlists/get_series.js").unwrap();
                }

                _ => {
                    println!("Unsupport find command.")
                }
            } //replace spaces and capitalize
        }
    }

    Ok(())
}
