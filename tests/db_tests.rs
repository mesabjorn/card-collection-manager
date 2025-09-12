use card_collection_manager::card::DatabaseCard;
use card_collection_manager::series::Series;

use card_collection_manager::db::setup;

#[test]
fn test_insert_and_get_series() {
    let db = setup(":memory:").unwrap(); // in-memory SQLite for testing

    let series = Series {
        id: None,
        name: "Test Series".into(),
        release_date: "2025-09-05".into(),
        n_cards: 10,
        prefix: Some("TS".into()),
    };

    let id = db.insert_series(&series).unwrap();
    assert!(id > 0);

    let fetched = db.get_unique_series().unwrap();
    assert_eq!(fetched.len(), 1);
    assert_eq!(fetched[0].name, "Test Series");
}

#[test]
fn test_insert_and_get_card() {
    let db = setup(":memory:").unwrap();

    let series = Series {
        id: None,
        name: "Test Series".into(),
        release_date: "2025-09-05".into(),
        n_cards: 10,
        prefix: Some("TS".into()),
    };
    let series_id = db.insert_series(&series).unwrap();

    let card = DatabaseCard {
        name: "Test Card".into(),
        series_id,
        number: "TS-001".into(),
        collection_number: 1,
        in_collection: 0,
        rarity_id: 1,
        card_type_id: 1,
    };

    let card_id = db.insert_card(&card).unwrap();
    assert!(card_id > 0);

    let cards = db.get_cards(None).unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].name, "Test Card");
}
