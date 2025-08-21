use chrono::NaiveDate;
use rusqlite::{Connection, Result, params};

use crate::card::Card;
use crate::dberror::DbError;
use crate::series::Series;

pub struct DatabaseConnection {
    conn: Connection,
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
            "CREATE TABLE IF NOT EXISTS series (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        release_date DATE NOT NULL,
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
                in_collection BOOLEAN NOT NULL DEFAULT 0,
                rarity_id INTEGER NOT NULL,
                FOREIGN KEY (rarity_id) REFERENCES rarity(id)
                FOREIGN KEY (series_id) REFERENCES series(id)
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

    pub fn insert_series(&self, series: &Series) -> Result<i32> {
        let release_date = NaiveDate::parse_from_str(&series.release_date, "%B %d, %Y")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

        self.conn.execute(
            "INSERT OR IGNORE INTO series (name, release_date, n_cards)
         VALUES (?1, ?2, ?3)",
            params![series.name, release_date.to_string(), series.n_cards],
        )?;

        // Always fetch the id (whether newly inserted or existing)
        let mut stmt = self.conn.prepare("SELECT id FROM series WHERE name = ?1")?;
        let id: i32 = stmt.query_row([&series.name], |row| row.get(0))?;
        Ok(id)
    }

    /// Insert card entry
    pub fn insert_card(&self, card: &Card) -> Result<i32, DbError> {
        // Get the series name safely
        let series = self.get_series_by_id(card.series_id)?;

        match self.conn.execute(
            "INSERT INTO cards (name, series_id, number, collection_number, in_collection, rarity_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                card.name,
                card.series_id,
                card.number,
                card.collection_number,
                card.in_collection,
                card.rarity_id
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

    pub fn collect_card(&self, card_id: &str) -> Result<(i32), DbError> {
        // Use a single SQL statement to increment and return the new value
        let new_count: i32 = self
            .conn
            .query_row(
                "UPDATE cards
         SET in_collection = in_collection + 1
         WHERE number = ?1
         RETURNING in_collection",
                params![card_id],
                |row| row.get(0),
            )
            .map_err(DbError::from)?; // convert rusqlite::Error to DbError if needed

        Ok(new_count)
    }

    pub fn set_card_collection_count(&self, card_id: &str, value: i32) -> Result<(), DbError> {
        self.conn.execute(
            "UPDATE cards SET in_collection = ?1 WHERE number = ?2",
            params![value, card_id],
        )?;
        Ok(())
    }

    /// Query cards with rarity name joined
    pub fn get_cards(&self) -> Result<Vec<(Card, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT c.name, c.series_id, c.number, c.collection_number, c.in_collection, c.rarity_id, r.name
             FROM cards c
             JOIN rarity r ON c.rarity_id = r.id",
        )?;

        let card_iter = stmt.query_map([], |row| {
            let card = Card {
                name: row.get(0)?,
                series_id: row.get(1)?,
                number: row.get(2)?,
                collection_number: row.get(3)?,
                in_collection: row.get(4)?,
                rarity_id: row.get(5)?,
            };
            let rarity_name: String = row.get(6)?;
            Ok((card, rarity_name))
        })?;

        Ok(card_iter.filter_map(Result::ok).collect())
    }

    /// Query cards with rarity name joined
    pub fn get_cards_by_series(
        &self,
        series_name: &str,
    ) -> Result<Vec<(Card, String, String)>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT c.name, c.number, c.collection_number, c.in_collection,
                r.name, s.name, c.series_id, c.rarity_id
         FROM cards c
         JOIN rarity r ON c.rarity_id = r.id
         JOIN series s ON c.series_id = s.id
         WHERE s.name = ?1",
            )
            .map_err(DbError::SqliteError)?;

        let card_iter = stmt
            .query_map([series_name], |row| {
                let card = Card {
                    name: row.get(0)?,
                    number: row.get(1)?,
                    collection_number: row.get(2)?,
                    in_collection: row.get(3)?,
                    rarity_id: row.get(7)?,
                    series_id: row.get(6)?,
                };
                let rarity_name: String = row.get(4)?;
                let series_name: String = row.get(5)?;
                Ok((card, rarity_name, series_name))
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
            .prepare("SELECT id, name, release_date, n_cards FROM series WHERE id = ?1")?;

        match stmt.query_row([id], |r| {
            Ok(Series {
                id: r.get(0)?,
                name: r.get(1)?,
                release_date: r.get(2)?,
                n_cards: r.get(3)?,
            })
        }) {
            Ok(series) => Ok(series),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                Err(DbError::UnknownSeries(id.to_string()))
            }
            Err(e) => Err(DbError::SqliteError(e)),
        }
    }
}
