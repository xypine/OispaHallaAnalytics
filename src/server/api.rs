use std::time::{SystemTime, UNIX_EPOCH};

use poem::web::Data;
use poem_openapi::{payload::Json, OpenApi, Tags};

use super::db::DbPool;

use twothousand_forty_eight::parser::parse_data;
use twothousand_forty_eight::recording::Recording;
use twothousand_forty_eight::validator::{validate_first_move, validate_history, ValidationData};

pub mod input_types;
pub mod internal_types;
pub mod response_types;
use input_types::*;
use internal_types::*;
use response_types::*;

#[derive(Tags)]
enum ApiTags {
    /// Information about the server
    Meta,
    /// Submit or get data
    Analytics,
}

#[derive(Default)]
pub struct Api;

#[OpenApi]
impl Api {
    /// Check if the server is online
    #[oai(path = "/alive", method = "get", tag = "ApiTags::Meta")]
    async fn alive(&self) -> AliveResponse {
        AliveResponse::Ok
    }
    /// Get information about the data gathered by the server
    #[oai(path = "/stats", method = "get", tag = "ApiTags::Meta")]
    async fn stats(&self, pool: Data<&DbPool>) -> StatsResponse {
        StatsResponse::Ok(Json(Stats {
            recorded_games: sqlx::query!("SELECT COUNT(*) as count from OHA")
                .fetch_one(pool.0)
                .await
                .unwrap()
                .count as usize,
        }))
    }
    /// Get information about the server
    #[oai(path = "/get_config", method = "get", tag = "ApiTags::Meta")]
    async fn get_config(&self) -> GetConfigResponse {
        pub mod built_info {
            // The file has been placed there by the build script.
            include!(concat!(env!("OUT_DIR"), "/built.rs"));
        }
        let version = built_info::PKG_VERSION;
        GetConfigResponse::Ok(Json(ServerConfig {
            platform: built_info::TARGET.to_string(),
            version: version.to_string(),
            rust_version: built_info::RUSTC_VERSION.to_string(),
        }))
    }

    /// Record a played game
    #[oai(path = "/record", method = "post", tag = "ApiTags::Analytics")]
    async fn record(&self, pool: Data<&DbPool>, input: Json<RecordInput>) -> RecordResponse {
        let run = input.r.clone();
        let client = input.client.clone();
        let result = self.parse_and_validate(run.clone()).await;
        match result {
            None => RecordResponse::Malformed,
            Some(recording) => {
                let hash = recording.hash_v1();
                if sqlx::query!("SELECT id from OHA where hash = ?", hash)
                    .fetch_optional(pool.0)
                    .await
                    .unwrap()
                    .is_some()
                {
                    RecordResponse::AlreadyExists
                } else {
                    let validation_result = validate_history(recording.clone());
                    if !validation_result.is_ok() {
                        return RecordResponse::InvalidGame;
                    }
                    let ValidationData {
                        valid,
                        score,
                        score_margin,
                        ..
                    } = validation_result.unwrap();
                    if !valid {
                        return RecordResponse::Malformed;
                    }
                    let _parsed = ParsedGame {
                        recording,
                        won: input.won.clone(),
                        abandoned: input.abandoned.clone(),
                        score: input.score,
                        computed_score: score,
                        computed_score_margin: score_margin,
                        timestamp_ms: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time is going backwards!")
                            .as_millis() as usize,
                    };
                    sqlx::query!(
                        r#"
                            INSERT INTO OHA(data_raw, client, hash) 
                            VALUES (?1, ?2, ?3)
                        "#,
                        run,
                        client,
                        hash,
                    )
                    .execute(pool.0)
                    .await
                    .unwrap();
                    RecordResponse::Ok
                }
            }
        }
    }

    async fn parse_and_validate(&self, run: String) -> Option<Recording> {
        let recording = parse_data(run.clone()).ok()?;
        if self.validate(&recording).await {
            return Some(recording);
        }
        None
    }

    async fn validate(&self, history: &Recording) -> bool {
        let length = history.history.len();
        println!("Loaded record with the length of {}.", length);
        let hash = history.hash_v1();
        let w = history.width;
        let h = history.height;
        let result0 = validate_first_move(&history);
        let Ok(ValidationData {
            valid,
            score,
            score_end: _,
            score_margin: _,
            breaks,
            break_positions: _,
        }) = validate_history(history.clone()) else {
            return false;
        };
        let valid = result0 && valid;
        println!("Run <{}>", hash);
        println!("\tBoard size: {}x{}", w, h);
        println!("\tRun score: {}", score);
        println!("\tBreaks used: {}", breaks);
        println!("\tValid: {}", valid);

        true
    }

    /// Get recorded games
    #[oai(path = "/data", method = "get", tag = "ApiTags::Analytics")]
    async fn get_data(&self, pool: Data<&DbPool>) -> GetDataResponse {
        let data = sqlx::query!("SELECT * from OHA")
            .fetch_all(pool.0)
            .await
            .unwrap();
        GetDataResponse::Ok(Json(DataWrapper {
            data: data
                .iter()
                .map(|r| {
                    serde_json::json!(Game {
                        id: r.id,
                        client: r.client.clone(),
                        data_raw: r.data_raw.clone(),
                        hash: r.hash.clone()
                    })
                })
                .collect(),
        }))
    }
}
