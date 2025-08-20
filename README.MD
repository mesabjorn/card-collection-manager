# Card Collection Manager

A command-line tool written in Rust for managing a collectible card database (e.g., Yu-Gi-Oh, Pokémon, etc.).  
It allows you to create series, add cards, import from JSON, and list your collection in custom formats.  

---

## ✨ Features
- Create and store **series** with name, release date, and number of cards.  
- Add **cards** manually or in bulk from a JSON file.  
- Manage **rarities** automatically (Common, Rare, Super Rare, etc.).  
- List all cards or filter cards by series.  
- Flexible output formatting with placeholders (`{name}`, `{number}`, `{rarity}`, etc.).  

---

## 📦 Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/yugioh-cards-list.git
   cd yugioh-cards-list

2. Build the project with Cargo:
`cargo build --release`
3. Run the binary:
`./target/release/yugioh-cards-list.exe --help`

# General Syntax
`yugioh-cards-list <DB_NAME> --modus <mode> [OPTIONS]`
## Supported modes:
1.  add_series: Add a new series interactively:  
`yugioh-cards-list mycards.db --modus add_series`  
Prompts you for: series name, release date, number of cards.
2. add_card: Add a new card interactively:  
`yugioh-cards-list mycards.db --modus add_card`
Prompts you for: card name, series number (e.g. LOB-EN000), series ID, rarity ID, collection number.
3. Import series & cards from JSON:  
`yugioh-cards-list mycards.db --modus add_json --filename cards.json`
4. List all cards:  
`yugioh-cards-list mycards.db --modus list_cards`
5. List cards by series:
`yugioh-cards-list mycards.db --modus list_series --series "Legend of Blue Eyes White Dragon"`  
5.1 With custom formatter:  
`yugioh-cards-list mycards.db --modus list_series --series "Legend of Blue Eyes White Dragon" --formatter "{series} - {number}: {name} [{rarity}]"`


