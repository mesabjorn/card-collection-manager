#[derive(Debug)]
pub struct Series {
    pub id: Option<i32>, // optional until inserted
    pub name: String,
    pub release_year: i32,
    pub n_cards: i32,
}
