use std::fmt;

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CardType {
    pub main: String,
    pub sub: String,
}

impl CardType {
    pub fn display(&self) -> String {
        format!("{} {}", self.sub, self.main)
    }
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.sub, self.main)
    }
}
