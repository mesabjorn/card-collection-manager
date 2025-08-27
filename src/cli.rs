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
        /// Kind of entity to add [series | cards | json]        
        kind: String,

        /// JSON file with cards (required for add json)
        #[arg(short, long)]
        filename: Option<String>,
    },

    /// List entities (series, cards)
    List {
        /// Kind of entity to list [series | cards]
        kind: String,

        /// series name filter (for list series)
        #[arg(long)]
        name: Option<String>,

        //if added hides card already in collection
        #[arg(long)]
        hide_collected: bool,

        /// Custom output formatter, e.g. "{name},{number},{rarity}"
        #[arg(long, default_value = "|{series}|{number}|{name}|")]
        formatter: String,
    },

    /// Collect a card
    Collect {
        /// Card ID to collect
        #[arg(long, num_args = 1..)]
        id: Vec<String>,

        /// If a single card is given, set `in_collection` to this value
        #[arg(long)]
        count: Option<i32>,
    },
    Find {
        /// Kind of entity to list [series | cards]
        kind: String,

        #[arg(long)]
        query: Option<String>,
    },
}
