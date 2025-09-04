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
   git clone https://github.com/mesabjorn/card-collection-manager.git
   cd card-collection-manager

2. Build the project with Cargo:
`cargo build --release`
3. Run the binary:
`./target/release/card-collection-manager.exe --help`

# General Syntax
`card-collection-manager <DB_NAME> <command> [OPTIONS]`
## Supported commands:
0. `init`: initializes the database.
1.  `add <kind>`:  
- kind can be one of card|series|json|rarity  
- card and series will be prompted interactively:  
`card-collection-manager mycards.db add card`  
`card-collection-manager mycards.db add series`  
- json requires --filename option to a json file:  
`card-collection-manager mycards.db add json --filename your_file.json`  
- rarity requires a name of the rarity:  
`card-collection-manager mycards.db add rarity "Another rarity level"`  
2. `list <kind>`: list contents from the databse:  
- kind can be one of cards|series|TODO:type|TODO:rarity
- cards: lists all cards in database  
`card-collection-manager mycards.db list cards`
- series: lists all unique series in database  
`card-collection-manager mycards.db list series`  
- serie: list cards from one serie:  
`card-collection-manager mycards.db list serie --name "Your Series Name" `
```bash 
Options:
      --name <NAME>            series name
      --hide-collected         hides collected items in output
      --formatter <FORMATTER>  Custom output formatter, e.g. "{name},{number},{rarity}" 
      Allowed options: 
         {name} = card name 
         {number} = card number 
         {collection_number} = unique collection id 
         {rarity} = rarity name 
         {series} = series name 
         {card_type} = card type 
         {in_collection} = copies in collection 
      [default: |{series}|{number}|{name}|]
```  
- type: lists all unique rarities:  
`card-collection-manager mycards.db list rarity` (Not yet implemented)
- type: lists all unique types in database  
`card-collection-manager mycards.db list type`  (Not yet implemented)  
3. Collect id: Adds cards to your collection:
- Collect a single card (add 1 to a certain card id):  
`card-collection-manager mycards.db collect --id id`  
Collect a multiple cards (add 1 to a certain card ids):  
`card-collection-manager mycards.db collect --id id1 id2 id3`  
Collect a multiple cards by range: (add 1 to all card ids in range):  
`card-collection-manager mycards.db collect --id range`  
Where range can be a series prefix with collection number(e.g. 'SDK-001-010'): collects all cards with id's SDK-001 through SDK-010 (adds one to each)
- Set collection count to a fixed number:  
`card-collection-manager mycards.db collect --id id --count <number>`
4. Sell id: Removes cards from your collection:
- Remove a single card (subtract 1 to a certain card id):  
`card-collection-manager mycards.db sell --id id`  
Sell multiple cards (subtract 1 to a certain card ids):  
`card-collection-manager mycards.db sell--id id1 id2 id3`  
Sell multiple cards by range: (subtract 1 to all card ids in range):  
`card-collection-manager mycards.db sell --id range`  
Where range can be a series prefix with collection number(e.g. 'SDK-001-010'): removes one of all cards with id's SDK-001 through SDK-010
5. Find cards or series:
```bash
Usage: card-collection-manager.exe <DBNAME> find [OPTIONS] <KIND>

Arguments:
  <KIND>  Kind of entity to list [series | cards]

Options:
      --query <QUERY> your search query
```
- find cards in your collection containing text 'query':  
 `card-collection-manager.exe <DBNAME> find cards --query query`
- find series on the [yugioh fandom](https://yugioh.fandom.com) like 'query':  
 `card-collection-manager.exe <DBNAME> find serie --query query`
 Also copies the contents of get_series.js to clipboard for easier scraping.
- TODO: find cards --type spell|trap|monster|effect|fusion etc
- TODO: find cards --rarity rare|super rare|ultra rare|etc
 

