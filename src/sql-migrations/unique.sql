-- Make cards table unique for the combination of (number, rarity_id) instead of just number
BEGIN TRANSACTION;

-- Rename old table
ALTER TABLE cards RENAME TO cards_old;

-- Create new table with correct schema
CREATE TABLE cards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    series_id INTEGER NOT NULL,
    collection_number INTEGER NOT NULL,
    number TEXT NOT NULL,
    in_collection INTEGER NOT NULL DEFAULT 0,
    rarity_id INTEGER NOT NULL,
    card_type_id INTEGER NOT NULL,
    FOREIGN KEY (rarity_id) REFERENCES rarity(id),
    FOREIGN KEY (series_id) REFERENCES series(id),
    FOREIGN KEY (card_type_id) REFERENCES card_type(id),
    UNIQUE (number, rarity_id)
);

-- Copy data over (may fail if duplicates exist!)
INSERT INTO cards (id, name, series_id, collection_number, number, in_collection, rarity_id, card_type_id)
SELECT id, name, series_id, collection_number, number, in_collection, rarity_id, card_type_id
FROM cards_old;

-- Drop old table
DROP TABLE cards_old;

COMMIT;
