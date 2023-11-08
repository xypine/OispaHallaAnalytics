use poem_openapi::{payload::Json, ApiResponse, Object};

#[derive(ApiResponse)]
pub enum AliveResponse {
    /// Returns when alive is requested.
    #[oai(status = 200)]
    Ok,
}

#[derive(Debug, Clone, Object)]
pub struct ServerConfig {
    pub platform: String,
    pub version: String,
    pub rust_version: String,
}

#[derive(ApiResponse)]
pub enum GetConfigResponse {
    /// Returns when get_config is requested.
    #[oai(status = 200)]
    Ok(Json<ServerConfig>),
}

#[derive(ApiResponse)]
pub enum WipeResponse {
    /// Returns when get_config is requested.
    #[oai(status = 200)]
    Ok,

    #[oai(status = 401)]
    Unauthorized,
}

#[derive(Debug, Clone, Object)]
pub struct Stats {
    pub recorded_games: usize,
}
#[derive(ApiResponse)]
pub enum StatsResponse {
    #[oai(status = 200)]
    Ok(Json<Stats>),
}

#[derive(ApiResponse)]
pub enum RecordResponse {
    /// Game recorded succesfully
    #[oai(status = 200)]
    Ok,

    /// Validating the game failed
    #[oai(status = 400)]
    InvalidGame,

    /// The run already exists
    #[oai(status = 409)]
    AlreadyExists,
}

#[derive(Debug, Clone, Object)]
pub struct Game {
    pub data_raw: String,
    pub hash: String,
    pub client: Option<String>,
    pub created_at: i64,
}

#[derive(ApiResponse)]
pub enum GetDataResponse {
    /// Return requested games
    #[oai(status = 200)]
    Ok(Json<Vec<Game>>),
}
