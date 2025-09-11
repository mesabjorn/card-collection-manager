use std::fmt;

#[derive(Debug)]
pub enum DbError {
    UnknownRarity(String),
    UnknownSeries(String),
    UnknownCardType(String),
    UniqueConstraintViolation(String),
    InvalidOperation(String),

    SqliteError(rusqlite::Error),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::UnknownRarity(name) => write!(f, "Encountered undefined rarity: {}", name),
            DbError::UnknownSeries(name) => write!(f, "Encountered undefined series: {}", name),
            DbError::UnknownCardType(name) => {
                write!(f, "Encountered undefined card type: {}", name)
            }
            DbError::InvalidOperation(name) => write!(f, "Invalid DB operation: {}", name),
            DbError::UniqueConstraintViolation(name) => write!(f, "Adding card failure: {}", name),

            DbError::SqliteError(e) => write!(f, "SQLite error: {}", e),
        }
    }
}

impl std::error::Error for DbError {}

impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        DbError::SqliteError(e)
    }
}
