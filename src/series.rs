use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Series {
    pub id: Option<i32>, // optional until inserted
    pub name: String,
    pub release_date: String,
    pub n_cards: i32,
    pub prefix: Option<String>,
}

impl std::ops::Deref for Series {
    type Target = Option<i32>;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
