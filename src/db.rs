use rusqlite::{Connection, Result, params};

use crate::card::Card;
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
        release_year INTEGER NOT NULL,
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
                number TEXT NOT NULL,
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
        // Try inserting
        self.conn.execute(
            "INSERT OR IGNORE INTO series (name, release_year, n_cards)
         VALUES (?1, ?2, ?3)",
            params![series.name, series.release_year, series.n_cards],
        )?;

        // Always fetch the id (whether newly inserted or existing)
        let mut stmt = self.conn.prepare("SELECT id FROM series WHERE name = ?1")?;
        let id: i32 = stmt.query_row([&series.name], |row| row.get(0))?;
        Ok(id)
    }

    /// Insert card entry
    pub fn insert_card(&self, card: &Card) -> Result<i32> {
        self.conn.execute(
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
        )?;
        Ok(self.conn.last_insert_rowid() as i32)
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
}
