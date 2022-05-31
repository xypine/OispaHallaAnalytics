use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::Mutex;
use poem_openapi::{Tags, OpenApi, payload::Json};

use twothousand_forty_eight::validator::{validate_first_move, validate_history};
use twothousand_forty_eight::parser::parse_data;
use twothousand_forty_eight::recording::Recording;


pub mod internal_types;
pub mod response_types;
pub mod input_types;
use internal_types::*;
use response_types::*;
use input_types::*;

#[derive(Tags)]
enum ApiTags {
    /// Information about the server
    Meta,
    /// Submit or get data
    Analytics,
}

#[derive(Default)]
pub struct Api {
    data: Mutex<Vec<Game>>,
    hashes: Mutex<Vec<String>>
}

#[OpenApi]
impl Api {
    /// Check if the server is online
    #[oai(path = "/alive", method = "get", tag = "ApiTags::Meta")]
    async fn alive(&self) -> AliveResponse {
        AliveResponse::Ok
    }
    /// Get information about the data gathered by the server
    #[oai(path = "/stats", method = "get", tag = "ApiTags::Meta")]
    async fn stats(&self) -> StatsResponse {
        StatsResponse::Ok(
            Json(Stats{
                recorded_games: self.data.lock().await.len()
            })
        )
    }
    /// Get information about the server
    #[oai(path = "/get_config", method = "get", tag = "ApiTags::Meta")]
    async fn get_config(&self) -> GetConfigResponse {
        pub mod built_info {
            // The file has been placed there by the build script.
            include!(concat!(env!("OUT_DIR"), "/built.rs"));
        }
        let version = built_info::PKG_VERSION;
        GetConfigResponse::Ok(
            Json(ServerConfig{
                platform: built_info::TARGET.to_string(),
                version: version.to_string(),
                rust_version: built_info::RUSTC_VERSION.to_string(),
            })
        )
    }
    
    /// Record a played game
    #[oai(path = "/record", method = "post", tag = "ApiTags::Analytics")]
    async fn record(&self, input: Json<RecordInput>) -> RecordResponse {
        let result = self.parse_and_validate(input.r.clone()).await;
        match result {
            None => RecordResponse::Malformed,
            Some( recording ) => {
                let mut data = self.data.lock().await;
                let mut hashes = self.hashes.lock().await;
                let hash = recording.hash_v1();
                if hashes.contains(&hash) {
                    RecordResponse::AlreadyExists
                }
                else {
                    let (_valid, computed_score, computed_socre_margin, _breaks) = validate_history(recording.clone());
                    data.push(
                        Game {
                            recording,
                            won: input.won,
                            abandoned: Some(input.abandoned),
                            score: input.score,
                            computed_score,
                            computed_socre_margin,
                            timestamp_ms: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time is going backwards!").as_millis() as usize

                        }
                    );
                    hashes.push(hash);
                    RecordResponse::Ok
                }
            },
        }
    }

    async fn parse_and_validate(&self, run: String) -> Option<Recording> {
        let recording = parse_data(run.clone());
        if self.validate(&recording, run).await {
            return recording;
        }
        None
    }

    async fn validate(&self, parsed: &Option<Recording>, run: String) -> bool {
        match parsed {
            None => {
                println!("Error while parsing run \"{}\"", run);
                // let mut error_count = self.error_count.lock().await;
                // *error_count += 1;
                false
            },
            Some( history ) => {
                let length = history.history.len();
                println!("Loaded record with the length of {}.", length);
                let hash = history.hash_v1();
                let w = history.width;
                let h = history.height;
                let result0 = validate_first_move(&history);
                let (result1, score, _score_margin, breaks) = validate_history(history.clone());
                let valid = result0 && result1;
                println!( "Run <{}>", hash );
                println!( "\tBoard size: {}x{}", w, h );
                println!( "\tRun score: {}", score );
                println!( "\tBreaks used: {}", breaks );
                println!( "\tValid: {}", valid );

                true
            },
        }
    }

    /// Get recorded games
    #[oai(path = "/data", method = "get", tag = "ApiTags::Analytics")]
    async fn get_data(&self) -> GetDataResponse {
        let data = self.data.lock().await;
        GetDataResponse::Ok(
            Json(
                Data {
                    data: data.iter().map(|r| serde_json::json!(r)).collect()
                }
            )
        )
    }
}
