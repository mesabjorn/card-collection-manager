use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rarity {
    pub id: i32,
    pub name: String,
}
