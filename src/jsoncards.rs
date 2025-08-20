use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CardJson {
    pub card_number: String,
    pub name: String,
    pub rarity: String,
    pub category: String,
}

#[derive(Deserialize)]
pub struct SeriesJson {
    pub name: String,
    pub ncards: i32,
    pub release_date: String,
    pub cards: Vec<CardJson>,
}
