use futures::future::join_all;
use futures_util::StreamExt;
use std::{collections::HashMap, io::Write, process::exit};
use tokio::task::JoinHandle;
use twothousand_forty_eight::unified::reconstruction::Reconstructable;

use crate::libproxy::{get_gamestate, get_moves, parse_data, reconstruct_history};
use crate::Game;
use rayon::prelude::*;
use sqlx::{query_builder, QueryBuilder, Sqlite, SqlitePool};
use twothousand_forty_eight::{
    direction::Direction,
    unified::{
        game::GameState, parse, reconstruct, reconstruction::HistoryReconstruction, validate,
        ParseResult,
    },
};

fn bool_to_r_str(boolean: bool) -> String {
    if boolean {
        String::from("TRUE")
    } else {
        String::from("FALSE")
    }
}

fn move_to_str(mv: Direction) -> String {
    return format!("{:?}", mv)
        .replace("UP", "Ylös")
        .replace("RIGHT", "Oikealle")
        .replace("DOWN", "Alas")
        .replace("LEFT", "Vasemmalle");
}

fn median(array: &Vec<usize>) -> f64 {
    if (array.len() % 2) == 0 {
        let ind_left = array.len() / 2 - 1;
        let ind_right = array.len() / 2;
        (array[ind_left] + array[ind_right]) as f64 / 2.0
    } else {
        array[(array.len() / 2)] as f64
    }
}

pub fn analyze_general(data: &Vec<Game>, filename: &str) {
    let recordings: Vec<ParseResult> = data
        .par_iter()
        .map(|i| parse_data(i.data_raw.clone()).unwrap())
        .collect();
    let reconstructions: Vec<HistoryReconstruction> = data
        .par_iter()
        .map(|i| reconstruct_history(i.data_raw.clone()).unwrap())
        .collect();

    let clients: Vec<String> = data
        .par_iter()
        .map(|i| i.client.clone().unwrap_or("unknown".to_owned()))
        .collect();
    let scores: Vec<usize> = data
        .par_iter()
        .enumerate()
        .map(|(i, _)| reconstructions[i - 1].validation_data.score)
        .collect();

    let mut to_write = vec![vec![
        "client".to_string(),
        "game_length".to_string(),
        "score".to_string(),
        "move_first".to_string(),
        "move_last".to_string(),
    ]];

    let data_to_write: Vec<_> = recordings
        .par_iter()
        .enumerate()
        .map(|(index, rec)| {
            let moves = get_moves(rec);

            let time = clients[index].to_string();
            let length = moves.len();
            let score = scores[index].to_string();
            let move_first = moves[0];
            let move_last = match moves[length - 1] {
                Direction::END => moves[length - 2],
                _ => moves[length - 1],
            };

            vec![
                time,
                length.to_string(),
                score,
                move_to_str(move_first),
                move_to_str(move_last),
            ]
        })
        .collect();

    to_write.extend(data_to_write);

    crate::io::write_csv(filename, to_write).expect("Failed to write csv");
}

pub fn analyze_frequence_move_first(data: &Vec<Game>, filename: &str) {
    let recordings: Vec<ParseResult> = data
        .par_iter()
        .map(|i| parse_data(i.data_raw.clone()).unwrap())
        .collect();

    let mut moves_first = vec![];
    for rec in recordings {
        let moves = get_moves(&rec);
        let move_first = moves[0];
        moves_first.push(move_first);
    }

    let c = moves_first.iter().count();
    let c_up = moves_first.iter().filter(|m| m == &&Direction::UP).count();
    let c_right = moves_first
        .iter()
        .filter(|m| m == &&Direction::RIGHT)
        .count();
    let c_down = moves_first
        .iter()
        .filter(|m| m == &&Direction::DOWN)
        .count();
    let c_left = moves_first
        .iter()
        .filter(|m| m == &&Direction::LEFT)
        .count();
    let to_write = vec![
        vec![
            "Ensimmäinen siirto".to_string(),
            "f".to_string(),
            "f%".to_string(),
        ],
        vec![
            "Ylös".to_string(),
            c_up.to_string(),
            (c_up as f64 / c as f64).to_string(),
        ],
        vec![
            "Oikealle".to_string(),
            c_right.to_string(),
            (c_right as f64 / c as f64).to_string(),
        ],
        vec![
            "Alas".to_string(),
            c_down.to_string(),
            (c_down as f64 / c as f64).to_string(),
        ],
        vec![
            "Vasemmalle".to_string(),
            c_left.to_string(),
            (c_left as f64 / c as f64).to_string(),
        ],
        vec![
            "Yhteensä".to_string(),
            c.to_string(),
            (c as f64 / c as f64).to_string(),
        ],
    ];
    crate::io::write_csv(filename, to_write).expect("Failed to write csv");
}

pub fn analyze_frequence_moves(data: &Vec<Game>, filename: &str) {
    let recordings: Vec<ParseResult> = data
        .par_iter()
        .map(|i| parse_data(i.data_raw.clone()).unwrap())
        .collect();

    let mut moves: Vec<Direction> = vec![];
    for rec in recordings {
        let mut rec_moves = get_moves(&rec);
        moves.append(&mut rec_moves);
    }

    let c = moves.len();
    let c_up = moves.iter().filter(|m| m == &&Direction::UP).count();
    let c_right = moves.iter().filter(|m| m == &&Direction::RIGHT).count();
    let c_down = moves.iter().filter(|m| m == &&Direction::DOWN).count();
    let c_left = moves.iter().filter(|m| m == &&Direction::LEFT).count();
    let to_write = vec![
        vec!["Suunta".to_string(), "f".to_string(), "f_per".to_string()],
        vec![
            "Ylös".to_string(),
            c_up.to_string(),
            (c_up as f64 / c as f64).to_string(),
        ],
        vec![
            "Oikealle".to_string(),
            c_right.to_string(),
            (c_right as f64 / c as f64).to_string(),
        ],
        vec![
            "Alas".to_string(),
            c_down.to_string(),
            (c_down as f64 / c as f64).to_string(),
        ],
        vec![
            "Vasemmalle".to_string(),
            c_left.to_string(),
            (c_left as f64 / c as f64).to_string(),
        ],
        vec![
            "Yhteensä".to_string(),
            c.to_string(),
            (c as f64 / c as f64).to_string(),
        ],
    ];
    crate::io::write_csv(filename, to_write).expect("Failed to write csv");
}

pub fn analyze_first_move_vs_score(data: &Vec<Game>, filename: &str) {
    let recordings: Vec<ParseResult> = data
        .par_iter()
        .map(|i| parse_data(i.data_raw.clone()).unwrap())
        .collect();
    let reconstructions: Vec<HistoryReconstruction> = data
        .par_iter()
        .map(|i| reconstruct_history(i.data_raw.clone()).unwrap())
        .collect();
    let gamestates: Vec<GameState> = data
        .par_iter()
        .map(|i| get_gamestate(i.data_raw.clone()).unwrap())
        .collect();

    let scores: Vec<usize> = data
        .par_iter()
        .enumerate()
        .map(|(i, _)| reconstructions[i - 1].validation_data.score)
        .collect();

    let mut moves_first = vec![];
    let mut sum_scores: HashMap<Direction, usize> = HashMap::new();
    for (frame_index, rec) in recordings.iter().enumerate() {
        let moves = get_moves(rec);

        let move_first = moves[0];
        let score = scores[frame_index];
        match sum_scores.get_mut(&move_first) {
            Some(val) => {
                *val += score;
            }
            None => {
                sum_scores.insert(move_first, score);
            }
        }
        moves_first.push(move_first);
    }

    let c_up = moves_first
        .par_iter()
        .filter(|m| m == &&Direction::UP)
        .count();
    let c_right = moves_first
        .par_iter()
        .filter(|m| m == &&Direction::RIGHT)
        .count();
    let c_down = moves_first
        .par_iter()
        .filter(|m| m == &&Direction::DOWN)
        .count();
    let c_left = moves_first
        .par_iter()
        .filter(|m| m == &&Direction::LEFT)
        .count();

    let s_up = *sum_scores.get(&Direction::UP).unwrap_or(&0);
    let s_right = *sum_scores.get(&Direction::RIGHT).unwrap_or(&0);
    let s_down = *sum_scores.get(&Direction::DOWN).unwrap_or(&0);
    let s_left = *sum_scores.get(&Direction::LEFT).unwrap_or(&0);
    let to_write = vec![
        vec![
            "Suunta".to_string(),
            "Pisteet".to_string(),
            "Pisteet_avg".to_string(),
        ],
        vec![
            "Ylös".to_string(),
            s_up.to_string(),
            (s_up as f64 / c_up as f64).to_string(),
        ],
        vec![
            "Oikealle".to_string(),
            s_right.to_string(),
            (s_right as f64 / c_right as f64).to_string(),
        ],
        vec![
            "Alas".to_string(),
            s_down.to_string(),
            (s_down as f64 / c_down as f64).to_string(),
        ],
        vec![
            "Vasemmalle".to_string(),
            s_left.to_string(),
            (s_left as f64 / c_left as f64).to_string(),
        ],
    ];
    crate::io::write_csv(filename, to_write).expect("Failed to write csv");
}

pub async fn unpack_games(pool: SqlitePool) {
    let mut handles: Vec<_> = vec![];
    // stream data from database
    let mut stream = sqlx::query!("SELECT hash, data_raw FROM games").fetch(&pool);
    let mut i = 0;
    while let Some(row) = stream.next().await {
        let row = row.unwrap();
        let hash = row.hash;
        if i % 100 == 0 {
            println!("Processing game {i}: {}", &hash);
        }
        let data = row.data_raw;
        //println!("\tParsing data...");
        let parsed = parse_data(data).expect("Failed to parse data");
        let moves = get_moves(&parsed);
        //println!("\tReconstructing history...");
        let history = match parsed {
            ParseResult::V1(v1) => v1.reconstruct().unwrap(),
            ParseResult::V2(v2) => v2.reconstruct().unwrap(),
        };
        let pool = pool.clone();

        let handle = tokio::spawn(async move {
            let validation = history.validation_data;
            //println!("\tSaving validation data to database...");
            save_validation(
                &pool,
                hash.clone(),
                validation.score as i64,
                validation.score_end as i64,
                validation.score_margin as i64,
                validation.breaks as i64,
            )
            .await;
            //println!("\tSaving {} moves to database...", moves.len());
            save_moves(&pool, hash.clone(), moves).await;
        });
        handles.push(handle);
        if i % 200 == 0 {
            /*println!(
                "{total} Waiting for {} database writes to finish...",
                handles.len()
            );*/
            join_all(handles).await;
            handles = vec![];
        }
        i += 1;
    }
    println!(
        "Waiting for the final {} database writes to finish...",
        handles.len()
    );
    join_all(handles).await;
    println!("Done!");
}

async fn save_validation(
    pool: &sqlx::SqlitePool,
    game_hash: String,
    score: i64,
    score_end: i64,
    score_margin: i64,
    breaks: i64,
) {
    sqlx::query!(
        "INSERT INTO validations (game_hash, score, score_end, score_margin, breaks) VALUES (?, ?, ?, ?, ?) ON CONFLICT DO NOTHING",
        game_hash,
        score,
        score_end,
        score_margin,
        breaks
    )
    .execute(pool)
    .await
    .unwrap();
}

async fn save_moves(pool: &sqlx::SqlitePool, game_hash: String, moves: Vec<Direction>) {
    let values: Vec<_> = moves
        .iter()
        .enumerate()
        .map(|(move_index, m)| {
            let dir_index = m.get_index() as i64;
            let move_index = move_index as i64;
            (dir_index, move_index)
        })
        .collect();

    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO moves (game_hash, direction, move_index)");

    query_builder.push_values(values, |mut b, (direction, move_index)| {
        b.push_bind(game_hash.clone())
            .push_bind(direction)
            .push_bind(move_index);
    });

    query_builder.push("ON CONFLICT DO NOTHING");

    let mut query = query_builder.build();

    query.execute(pool).await.unwrap();
}
