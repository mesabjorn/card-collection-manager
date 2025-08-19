#[derive(Debug)]
pub struct Card {
    pub name: String,
    pub series_id: i32,
    pub number: String,
    pub collection_number: i32,
    pub in_collection: bool,
    pub rarity_id: i32,
}
