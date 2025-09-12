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
