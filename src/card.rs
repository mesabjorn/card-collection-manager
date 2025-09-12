use serde::{Deserialize, Serialize};

use crate::{cardtype::CardType, rarity::Rarity, series::Series};
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct DatabaseCard {
    pub name: String,
    pub series_id: i32,
    pub number: String,
    pub collection_number: i32,
    pub in_collection: i32,
    pub rarity_id: i32,
    pub card_type_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    pub name: String,
    pub number: String,
    pub collection_number: i32,
    pub in_collection: i32,
    pub series: Series,
    pub rarity: Rarity,
    pub cardtype: CardType,
}
