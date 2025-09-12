use std::error::Error;

use chrono::NaiveDate;
use rusqlite::{Connection, Result, params};

use crate::card::{Card, DatabaseCard};
use crate::cardtype::CardType;
use crate::dberror::DbError;

use crate::rarity::Rarity;
use crate::series::Series;

use rusqlite::OptionalExtension; // <- import this

pub struct DatabaseConnection {
    conn: Connection,
}

// Helper function to parse a card range like "LOB-001-010"
fn parse_card_range(card_id: &str) -> Option<(&str, Option<&str>, i32, i32)> {
    // Expect format: PREFIX-START-END
    // START and END can be numeric or series+number (like EN001)
    // Returns: (prefix, optional series prefix, start_number, end_number)

    let parts: Vec<&str> = card_id.split('-').collect();
    if parts.len() != 3 {
        return None;
    }

    let prefix = parts[0];
    let start_part = parts[1];
    let end_part = parts[2];

    // Extract series prefix and numeric value from start
    let start_series_end = start_part.find(|c: char| c.is_digit(10))?;
    let series_prefix = if start_series_end > 0 {
        Some(&start_part[..start_series_end])
    } else {
        None
    };
    let start_num = start_part[start_series_end..].parse::<i32>().ok()?;
    let end_num = end_part[start_series_end..].parse::<i32>().ok()?; // assume same series prefix for end

    Some((prefix, series_prefix, start_num, end_num))
}

pub fn get_series_and_number(s: &str) -> (String, i32) {
    // Returns tuple of (series_prefix, collection_number)
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

impl DatabaseConnection {
    /// Open (or create) a database file
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    /// Create required tables
    pub fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS rarity (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS card_type (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                maintype TEXT NOT NULL,  
                subtype TEXT,
                UNIQUE (maintype, subtype) -- unique constraint for the two columns together
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS series (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        release_date DATE NOT NULL,
        prefix TEXT,
        n_cards INTEGER NOT NULL DEFAULT 0
        )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS cards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                series_id INTEGER NOT NULL,
                collection_number INTEGER NOT NULL,
                number TEXT NOT NULL UNIQUE,
                in_collection INTEGER NOT NULL DEFAULT 0,
                rarity_id INTEGER NOT NULL,
                card_type_id INTEGER NOT NULL,
                FOREIGN KEY (rarity_id) REFERENCES rarity(id)
                FOREIGN KEY (series_id) REFERENCES series(id)
                FOREIGN KEY (card_type_id) REFERENCES card_type(id)
            )",
            [],
        )?;
        Ok(())
    }

    /// Insert rarity entry
    pub fn insert_rarity(&self, name: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO rarity (name) VALUES (?1)",
            params![name],
        )?;
        Ok(())
    }

    pub fn insert_card_type(&self, main_type: &str, sub_type: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO card_type (maintype,subtype) VALUES (?1,?2)",
            params![main_type, sub_type],
        )?;
        Ok(())
    }

    pub fn insert_series(&self, series: &Series) -> Result<i32> {
        let release_date = NaiveDate::parse_from_str(&series.release_date, "%B %d, %Y")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

        self.conn.execute(
            "INSERT OR IGNORE INTO series (name, release_date, n_cards,prefix)
         VALUES (?1, ?2, ?3,?4)",
            params![
                series.name,
                release_date.to_string(),
                series.n_cards,
                series.prefix
            ],
        )?;

        // Always fetch the id (whether newly inserted or existing)
        let mut stmt = self.conn.prepare("SELECT id FROM series WHERE name = ?1")?;
        let id: i32 = stmt.query_row([&series.name], |row| row.get(0))?;
        Ok(id)
    }

    /// Insert card entry
    pub fn insert_card(&self, card: &DatabaseCard) -> Result<i32, DbError> {
        // Get the series name safely
        let series = self.get_series_by_id(card.series_id)?;

        match self.conn.execute(
            "INSERT INTO cards (name, series_id, number, collection_number, in_collection, rarity_id,card_type_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                card.name,
                card.series_id,
                card.number,
                card.collection_number,
                card.in_collection,
                card.rarity_id,
                card.card_type_id
            ],
        ) {
            Ok(_) => Ok(self.conn.last_insert_rowid() as i32),
            Err(rusqlite::Error::SqliteFailure(e, _)) if e.extended_code == 2067 => {
            println!("Warning: Card '{}' already exists in series '{}'.", card.number, series.name);
            Ok(0) // indicate nothing was inserted
        }
            Err(e) => Err(DbError::SqliteError(e)),
        }
    }

    pub fn collect_card(&self, card_id: &str, count: Option<i32>) -> Result<i32, DbError> {
        // Check if the card_id contains a range (e.g., "LOB-001-010")
        if let Some((prefix, series_prefix, start, end)) = parse_card_range(card_id) {
            // Update all cards in the range
            let mut total_updated = 0;
            for num in start..=end {
                let card_number = format!(
                    "{}-{}{:03}",
                    prefix,
                    series_prefix.unwrap_or(""), // use empty string if None
                    num
                );
                let new_count: i32 = if let Some(c) = count {
                    println!("Collecting {} copies of '{}'", card_number, c);
                    self.conn
                        .query_row(
                            "UPDATE cards
                            SET in_collection = in_collection+?1
                            WHERE number = ?2
                            RETURNING in_collection",
                            params![c, card_number],
                            |row| row.get(0),
                        )
                        .map_err(DbError::from)?
                } else {
                    self.conn
                        .query_row(
                            "UPDATE cards
                        SET in_collection = in_collection + 1
                        WHERE number = ?1
                        RETURNING in_collection",
                            params![card_number],
                            |row| row.get(0),
                        )
                        .map_err(DbError::from)?
                };
                total_updated += new_count;
            }
            return Ok(total_updated);
        }

        let final_count = count.unwrap_or_else(|| 1); //default to increment by one

        //no count specified, add 1 to existing collection
        let new_count: i32 = self
            .conn
            .query_row(
                "UPDATE cards
                    SET in_collection = in_collection + ?1
                    WHERE number = ?2
                    RETURNING in_collection",
                params![final_count, card_id],
                |row| row.get(0),
            )
            .map_err(DbError::from)?; // convert rusqlite::Error to DbError if needed
        Ok(new_count)
    }

    pub fn sell_card(&self, card_id: &str, count: i32) -> Result<i32, DbError> {
        // Helper closure to update a single card
        let sell_single = |conn: &rusqlite::Connection, number: &str| -> Result<i32, DbError> {
            let new_count: Option<i32> = conn
                .query_row(
                    "UPDATE cards
                 SET in_collection = in_collection - ?2
                 WHERE number = ?1 AND in_collection -?2>=0
                 RETURNING in_collection",
                    params![number, count],
                    |row| row.get(0),
                )
                .optional() // returns None if no rows updated
                .map_err(DbError::from)?;

            match new_count {
                Some(count) => Ok(count),
                None => Err(DbError::InvalidOperation(format!(
                    "Could not sell card '{}': Number of copies in collection cannot become negative.",
                    number
                ))),
            }
        };

        // Range case (e.g., "LOB-001-010")
        if let Some((prefix, series_prefix, start, end)) = parse_card_range(card_id) {
            let mut total_updated = 0;

            for num in start..=end {
                let card_number = format!(
                    "{}-{}{:03}",
                    prefix,
                    series_prefix.unwrap_or(""), // use empty string if None
                    num
                );
                println!("Selling '{}'", card_number);
                total_updated += sell_single(&self.conn, &card_number)?;
            }

            return Ok(total_updated);
        }

        // Single card case
        sell_single(&self.conn, card_id)
    }

    /// Query cards with rarity name joined
    pub fn get_cards(&self, query: Option<&str>) -> Result<Vec<Card>> {
        let pattern = match query {
            Some(q) => format!("%{}%", q),
            None => "%".to_string(), // matches everything
        };

        let mut stmt = self.conn.prepare(
            "SELECT 
                    c.name, c.series_id, c.number, c.collection_number, c.in_collection, 
                    c.rarity_id, c.card_type_id, r.name, t.maintype, t.subtype,
                    s.name, s.prefix, s.release_date, s.n_cards
                    FROM cards c
                    JOIN rarity r ON c.rarity_id = r.id
                    JOIN card_type t ON c.card_type_id = t.id
                    JOIN series s ON c.series_id = s.id
                    where c.name LIKE ?1
                    COLLATE NOCASE
                    ",
        )?;

        let card_iter = stmt.query_map([pattern], |row| {
            let rarity = Rarity {
                id: row.get(5)?,
                name: row.get(7)?,
            };

            let series = Series {
                id: row.get(1)?,
                name: row.get(10)?,
                prefix: row.get(11)?,
                release_date: row.get(12)?,
                n_cards: row.get(13)?,
            };
            let card = Card {
                name: row.get(0)?,
                number: row.get(2)?,
                collection_number: row.get(3)?,
                in_collection: row.get(4)?,
                rarity: rarity,
                cardtype: CardType {
                    main: row.get(8)?,
                    sub: row.get(9)?,
                },
                series: series,
            };
            Ok(card)
        })?;

        let cards: Vec<Card> = card_iter.filter_map(Result::ok).collect();
        Ok(cards)
    }

    /// Query cards with rarity name joined
    pub fn get_cards_by_seriesname(&self, series_name: &str) -> Result<Vec<Card>, DbError> {
        let sql = "SELECT 
                                c.name, c.series_id, c.number, c.collection_number, c.in_collection, 
                                c.rarity_id, c.card_type_id, r.name, t.maintype, t.subtype,
                                s.name, s.prefix, s.release_date, s.n_cards
                                FROM cards c
                                JOIN rarity r ON c.rarity_id = r.id
                                JOIN card_type t ON c.card_type_id = t.id
                                JOIN series s ON c.series_id = s.id
                                where s.name = ?1
                                COLLATE NOCASE";
        let mut stmt = self.conn.prepare(sql)?;
        // println!("Executing SQL:\n{}\nWith param: '{}'", sql, series_name);

        // println!("'{:?}'", stmt);
        let card_iter = stmt
            .query_map([series_name], |row| {
                let rarity = Rarity {
                    id: row.get(5)?,
                    name: row.get(7)?,
                };

                let series = Series {
                    id: row.get(1)?,
                    name: row.get(10)?,
                    prefix: row.get(11)?,
                    n_cards: row.get(13)?,
                    release_date: row.get(12)?,
                };
                let card = Card {
                    name: row.get(0)?,
                    number: row.get(2)?,
                    collection_number: row.get(3)?,
                    in_collection: row.get(4)?,
                    rarity: rarity,
                    cardtype: CardType {
                        main: row.get(8)?,
                        sub: row.get(9)?,
                    },
                    series: series,
                };
                Ok(card)
            })
            .map_err(DbError::SqliteError)?;

        let results: Vec<_> = card_iter.filter_map(Result::ok).collect();

        if results.is_empty() {
            Err(DbError::UnknownSeries(series_name.to_string()))
        } else {
            Ok(results)
        }
    }

    pub fn get_rarity_id(&self, rarity_name: &str) -> Result<i32, DbError> {
        let mut stmt = self.conn.prepare("SELECT id FROM rarity WHERE name = ?1")?;
        match stmt.query_row([rarity_name], |r| r.get(0)) {
            Ok(id) => Ok(id),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                Err(DbError::UnknownRarity(rarity_name.into()))
            }
            Err(e) => Err(DbError::SqliteError(e)),
        }
    }

    pub fn get_series_by_id(&self, id: i32) -> Result<Series, DbError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, release_date, n_cards,prefix FROM series WHERE id = ?1")?;

        match stmt.query_row([id], |r| {
            Ok(Series {
                id: r.get(0)?,
                name: r.get(1)?,
                release_date: r.get(2)?,
                n_cards: r.get(3)?,
                prefix: r.get(4)?,
            })
        }) {
            Ok(series) => Ok(series),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                Err(DbError::UnknownSeries(id.to_string()))
            }
            Err(e) => Err(DbError::SqliteError(e)),
        }
    }

    pub fn get_card_type_id(&self, name: &str) -> Result<i32, DbError> {
        //expect names like "Continuous Spell Card" or "Effect Fusion Monster"
        // take first word as subtype
        // take all the rest as main type:
        // "Continuous Spell Card" -> maintype: Spell Card, subtype: Continuous
        // "Effect Fusion Monster" -> maintype: Fusion Monster, subtype: Effect

        let (subtype, maintype);

        if name == "Fusion Monster" {
            subtype = "Normal";
            maintype = "Fusion Monster";
        } else if name == "Ritual Monster" {
            subtype = "Normal";
            maintype = "Ritual Monster";
        } else {
            let mut parts = name.splitn(2, ' '); // split into at most 2 parts
            subtype = parts.next().unwrap_or("");
            maintype = parts.next().unwrap_or("");
        }
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM card_type WHERE maintype = ?1 and subtype = ?2")?;
        match stmt.query_one([maintype, subtype], |r| Ok(r.get(0)?)) {
            Ok(id) => Ok(id),
            Err(rusqlite::Error::QueryReturnedNoRows) => Err(DbError::UnknownCardType(format!(
                "{},{}",
                maintype, subtype
            ))),
            Err(e) => Err(DbError::SqliteError(e)),
        }
    }

    pub fn get_unique_series(&self) -> Result<Vec<Series>, DbError> {
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT id, name, n_cards, release_date,prefix FROM series order by release_date")?;

        let series_iter = stmt.query_map([], |row| {
            Ok(Series {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                n_cards: row.get(2)?,
                release_date: row.get(3)?,
                prefix: row.get(4)?,
            })
        })?;

        Ok(series_iter.filter_map(Result::ok).collect())
    }
}

pub fn setup(dbname: &str) -> Result<DatabaseConnection, Box<dyn Error>> {
    let db = DatabaseConnection::new(dbname)?;
    db.create_tables()?;

    // Insert rarities
    db.insert_rarity("Common")?;
    db.insert_rarity("Rare")?;
    db.insert_rarity("Super Rare")?;
    db.insert_rarity("Ultra Rare")?;
    db.insert_rarity("Secret Rare")?;
    db.insert_rarity("Starlight Rare")?;
    db.insert_rarity("Quarter Century Rare")?;

    // Insert card types
    db.insert_card_type("Spell Card", "Normal")?;
    db.insert_card_type("Spell Card", "Equip")?;
    db.insert_card_type("Spell Card", "Field")?;
    db.insert_card_type("Spell Card", "Quick-Play")?;
    db.insert_card_type("Monster", "Normal")?;
    db.insert_card_type("Monster", "Flip")?;
    db.insert_card_type("Monster", "Effect")?;
    db.insert_card_type("Monster", "Union")?;
    db.insert_card_type("Fusion Monster", "Normal")?;
    db.insert_card_type("Fusion Monster", "Effect")?;
    db.insert_card_type("Trap Card", "Normal")?;
    db.insert_card_type("Trap Card", "Continuous")?;
    db.insert_card_type("Trap Card", "Counter")?;

    Ok(db)
}
