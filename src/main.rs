use std::{
    error::Error,
    io::{self, BufReader, Write},
};

use open;

use crate::{
    card::Card,
    cli::{Args, Command},
    copy::add_file_to_clipboard,
    db::{DatabaseConnection, setup},
    series::Series,
};

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

    let mut abbreviation = String::new();
    print!("Enter abbreviated name (e.g. LOB or MRD): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut abbreviation).unwrap();
    let abbreviation = abbreviation.trim().to_string();

    Ok(Series {
        id: None,
        name,
        release_date,
        n_cards,
        abbreviation: Some(abbreviation.clone()),
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
    print!("Enter series id (numeric): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut series_id).unwrap();
    let series_id: i32 = series_id.trim().parse().unwrap_or(0);

    let mut rarity_id = String::new();
    print!("Enter rarity id (numeric): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut rarity_id).unwrap();
    let rarity_id: i32 = rarity_id.trim().parse().unwrap_or(0);

    let mut collection_number = String::new();
    print!("Enter collection number (numeric): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut collection_number).unwrap();
    let collection_number: i32 = collection_number.trim().parse().unwrap_or(0);

    let mut card_type_id = String::new();
    print!("Enter rarity id (numeric): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut card_type_id).unwrap();
    let card_type_id: i32 = card_type_id.trim().parse().unwrap_or(0);

    Ok(Card {
        name,
        series_id,
        number,
        collection_number: collection_number,
        in_collection: 0,
        rarity_id,
        card_type_id,
    })
}

fn format_card(
    card: &Card,
    rarity: &str,
    series: &str,
    card_type: &str,
    formatter: &str,
) -> String {
    formatter
        .replace("{name}", &card.name)
        .replace("{number}", &card.number)
        .replace("{collection_number}", &card.collection_number.to_string())
        .replace("{rarity}", rarity)
        .replace("{series}", series)
        .replace("{card_type}", card_type)
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
                    let filename = filename.expect("--filename is required for 'add json'");

                    let file = std::fs::File::open(filename)?;
                    let reader = BufReader::new(file);

                    let series_json: jsoncards::SeriesJson = serde_json::from_reader(reader)?;

                    // Insert series
                    let series = Series {
                        id: None,
                        name: series_json.name.clone(),
                        release_date: series_json.release_date,
                        n_cards: series_json.ncards,
                        abbreviation: Some(series_json.abbreviation.unwrap_or(String::from(""))),
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
                            card_type_id: db.get_card_type_id(&c.category)?,
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
                "card_type" => {
                    let n = name.expect("--name for card-type is required");
                    let mut parts = n.splitn(2, ' '); // split into at most 2 parts
                    let subtype = parts.next().unwrap_or(""); //first part is subtype e.g. EFFECT
                    let maintype = parts.next().expect("main type cannot be empty.");
                    db.insert_card_type(maintype, subtype)?;
                    println!("Inserted card type '{}'", n);
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
                    for (card, rarity, series, card_type) in cards {
                        if hide_collected && card.in_collection > 0 {
                            continue;
                        }
                        println!(
                            "{}",
                            format_card(&card, &rarity, &series, &card_type, &formatter)
                        );
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
                            "{}. {} | {} | {} cards",
                            cnt, s.name, s.release_date, s.n_cards
                        );
                        cnt += 1;
                    }
                }
                "rarities" => {
                    println!("Not yet implemented...");
                }
                "cardtypes" => {
                    println!("Not yet implemented...");
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
        Command::Sell { id } => {
            //for collecting card id's (e.g. PSV-EN001)
            if id.len() == 1 {
                let card_id = &id[0];

                let new_count = db.sell_card(&card_id)?;

                println!(
                    "Card removed. Card '{}' now has {} copies in collection.",
                    card_id, new_count
                );
            } else {
                for card_id in id {
                    let new_count = db.sell_card(&card_id)?;
                    println!(
                        "Card removed. Card {} now has {} copies in collection.",
                        card_id, new_count
                    );
                }
            }
        }
        Command::Find { kind, query } => {
            match kind.as_str() {
                "cards" => {
                    let q = query.expect("--query is required for 'find cards --query query'");
                    let cards = db.get_cards(Some(q.as_str()))?;
                    for (card, rarity) in cards {
                        println!("{:?} | Rarity: {}", card, rarity);
                    }
                }
                "serie" => {
                    let q = query.expect("--query is required for 'find serie --query query'");
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
                    println!("Unsupport find command. Use 'find cards|serie --query query'")
                }
            } //replace spaces and capitalize
        }
    }

    Ok(())
}
