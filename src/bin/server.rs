use axum::{Router, serve};
use card_collection_manager::db::setup;
use card_collection_manager::{AppState, api};

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "app", version, about = "Card DB Server")]
pub struct Args {
    /// Database file name    
    pub dbname: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse(); // Parse CLI arguments

    let conn = setup(&args.dbname).expect("failed to setup database");

    let state = Arc::new(AppState {
        db: Arc::new(Mutex::new(conn)), // wrap DatabaseConnection in Mutex + Arc
    });

    let app = Router::new().nest("/api", api::routes()).with_state(state);

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    println!("🚀 Server running at http://{}", addr);

    // 👇 new Axum 0.8 style
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
