use std::pin::Pin;

use analyzer::unpack_games;
use futures::future::join_all;
use futures_util::StreamExt;
use tokio::task::JoinHandle;
use tokio_retry::strategy::{jitter, ExponentialBackoff};
use tokio_retry::Retry;

use crate::filter::get_filtered_data;

mod analyzer;
mod filter;
mod io;
mod libproxy;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(
            std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set")
                .as_str(),
        )
        .await
        .unwrap();
    setup_db(&pool).await;
    save_games_to_db(&pool).await;
    unpack_games(pool).await;

    /*
    println!("Analyzing data...");
    println!("\tCalculating general data...");
    analyzer::analyze_general(&data, "out_all.csv");
    println!("\tAnalyzing first move frequencies...");
    analyzer::analyze_frequence_move_first(&data, "out_frequency_move_first.csv");
    println!("\tAnalyzing move frequencies in general...");
    analyzer::analyze_frequence_moves(&data, "out_frequency_moves.csv");
    println!("\tAnalyzing first move v score...");
    analyzer::analyze_first_move_vs_score(&data, "out_first_move_vs_score.csv");
    */
}

async fn setup_db(pool: &sqlx::SqlitePool) {
    sqlx::query("PRAGMA journal_mode=WAL")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("PRAGMA busy_timeout=60000")
        .execute(pool)
        .await
        .unwrap();
}

async fn save_games_to_db(pool: &sqlx::SqlitePool) {
    let data = get_filtered_data();
    io::write_composite(&data, "out_composite.txt");
    println!("Writing to database...");
    let mut handles: Vec<_> = vec![];
    let retry_strategy = ExponentialBackoff::from_millis(10)
        .map(jitter) // add jitter to delays
        .take(3); // limit to 3 retries

    let total = data.len();
    for (i, game) in data.iter().enumerate() {
        let conn = pool.clone();
        let game = game.clone();
        let handle = tokio::spawn(async move {
            save_game(&game, &conn).await;
        });
        handles.push(handle);
        if i % 400 == 0 {
            println!(
                "{i}/{total} Waiting for {} database writes to finish...",
                handles.len()
            );
            join_all(handles).await;
            handles = vec![];
        }
    }
    println!(
        "Waiting for the final {} database writes to finish...",
        handles.len()
    );
    join_all(handles).await;
    println!("Done!");
    drop(data);
}

async fn save_game(game: &Game, pool: &sqlx::SqlitePool) {
    let version = game.client.clone().unwrap_or_else(|| "unknown".to_string());
    sqlx::query!(
        "INSERT INTO games (client, data_raw, hash, created_at) VALUES (?, ?, ?, ?) ON CONFLICT DO NOTHING",
        version,
        game.data_raw,
        game.hash,
        game.created_at
    )
    .execute(pool)
    .await
    .unwrap();
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Game {
    pub data_raw: String,
    // pub data_parsed: String,
    pub hash: String,
    pub client: Option<String>,
    pub created_at: Option<i64>,
}
