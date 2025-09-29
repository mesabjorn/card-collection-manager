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

#[test]

fn test_collect_card_one_copy() {
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

    db.insert_card(&card).unwrap();

    let _card_id = db.collect_card("TS-001", None, None);

    let cards = db.get_cards(Some("Test Card")).unwrap();

    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].in_collection, 1);
}

#[test]

fn test_collect_card_five_copies() {
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
        in_collection: 0, // start with zero
        rarity_id: 1,
        card_type_id: 1,
    };

    db.insert_card(&card).unwrap();

    let _card_id = db.collect_card("TS-001", None, Some(5)); //add 5 copies

    let _sell = db.collect_card("TS-001", None, Some(-2)); //collect -2, gives 5-2 = 3 copies

    let cards = db.get_cards(Some("Test Card")).unwrap();

    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].in_collection, 3);
}

#[test]

fn test_sell_too_many() {
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
        in_collection: 5, // start with 5
        rarity_id: 1,
        card_type_id: 1,
    };

    db.insert_card(&card).unwrap();

    //attempt to sell 10
    let result = db.collect_card("TS-001", None, Some(-10));
    //cannot sell this much, so assert for error

    // assert that selling more than owned returns an error
    assert!(
        result.is_err(),
        "Expected error when selling too many, but got {:?}",
        result
    );
}
