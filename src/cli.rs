use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "app", version, about = "Card DB CLI")]
pub struct Args {
    /// Database file name    
    pub dbname: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init {},

    /// Add entities (series, cards, json)
    Add {
        /// Kind of entity to add [series | cards | json | rarity| card-type]
        kind: String,

        /// JSON file with cards (required for add json)
        #[arg(short, long)]
        filename: Option<String>,

        /// name for add rarity or card-type
        name: Option<String>,
    },

    /// List entities (series, cards)
    List {
        /// Kind of entity to list [series | cards | serie]
        kind: String,

        /// series name filter (for list serie)
        #[arg(long)]
        name: Option<String>,

        //hides card already in collection (defaults to false)
        #[arg(long)]
        hide_collected: bool,

        /// Custom output formatter, e.g. "{name},{number},{rarity}"
        /// Format options:
        /// {name}=card name
        /// {number}=card number
        /// {collection_number}=unique collection id
        /// {rarity}=rarity name
        /// {series}=series name
        /// {card_type}=card type
        /// {in_collection}=copies in collection
        #[arg(long, default_value = "|{series}|{number}|{name}|")]
        formatter: String,
    },

    /// Collect a card
    Collect {
        /// Card ID to collect
        #[arg(long, num_args = 1..)]
        id: Vec<String>,

        /// Set all cards to this number of cards in the collection
        #[arg(long)]
        count: Option<i32>,
    },
    /// Sell a card
    Sell {
        /// Card ID to sell
        #[arg(long, num_args = 1..)]
        id: Vec<String>,
    },
    Find {
        /// Kind of entity to list [serie | cards]
        kind: String,

        #[arg(long)]
        query: Option<String>,
    },
}
