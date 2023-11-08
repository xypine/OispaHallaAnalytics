use std::time::{SystemTime, UNIX_EPOCH};

use poem::web::Data;
use poem_openapi::{param::Query, payload::Json, OpenApi, Tags};

use super::db::DbPool;

use twothousand_forty_eight::unified::{hash, validate};

pub mod input_types;
pub mod internal_types;
pub mod response_types;
use input_types::*;
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
                .count
                .unwrap_or_default() as usize,
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

    #[oai(path = "/wipe", method = "post", tag = "ApiTags::Analytics")]
    async fn wipe(&self, pool: Data<&DbPool>, input: Json<WipeInput>) -> WipeResponse {
        let wipe_key = match std::env::var("WIPE_KEY") {
            Ok(v) => v,
            Err(_) => {
                println!("Attempted wipe with no key set");
                return WipeResponse::Unauthorized;
            }
        };
        if input.key != wipe_key {
            println!("Attempted wipe with invalid key: {}", input.key);
            return WipeResponse::Unauthorized;
        }
        sqlx::query!("DELETE FROM OHA")
            .execute(pool.0)
            .await
            .unwrap();
        WipeResponse::Ok
    }

    /// Record a played game
    #[oai(path = "/record", method = "post", tag = "ApiTags::Analytics")]
    async fn record(&self, pool: Data<&DbPool>, input: Json<RecordInput>) -> RecordResponse {
        let run = input.r.clone();
        let client = input.client.clone();
        let hash = hash(&run);
        if !hash.is_ok() {
            return RecordResponse::InvalidGame;
        }
        let hash = hash.unwrap();
        if sqlx::query!("SELECT hash from OHA where hash = $1", hash)
            .fetch_optional(pool.0)
            .await
            .unwrap()
            .is_some()
        {
            RecordResponse::AlreadyExists
        } else {
            let validation_result = validate(&run);
            if !validation_result.is_ok() {
                return RecordResponse::InvalidGame;
            }
            let created_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            sqlx::query!(
                r#"
                            INSERT INTO OHA(data_raw, client, hash, created_at) 
                            VALUES ($1, $2, $3, $4)
                        "#,
                run,
                client,
                hash,
                created_at
            )
            .execute(pool.0)
            .await
            .unwrap();
            RecordResponse::Ok
        }
    }

    /// Get recorded games
    #[oai(path = "/data", method = "get", tag = "ApiTags::Analytics")]
    async fn get_data(
        &self,
        pool: Data<&DbPool>,
        page: Query<Option<i64>>,
        page_size: Query<Option<i64>>,
    ) -> GetDataResponse {
        let page = page.unwrap_or(0);
        let page_size = page_size.unwrap_or(i64::MAX);
        let page_offset = page * page_size;
        let data = sqlx::query!(
            "SELECT * from OHA ORDER BY created_at DESC OFFSET $1 LIMIT $2",
            page_offset,
            page_size
        )
        .fetch_all(pool.0)
        .await
        .expect("Failed to fetch data");
        GetDataResponse::Ok(Json(
            data.iter()
                .map(|r| response_types::Game {
                    client: Some(r.client.clone()),
                    data_raw: r.data_raw.clone(),
                    hash: r.hash.clone(),
                    created_at: r.created_at,
                })
                .collect(),
        ))
    }
}
