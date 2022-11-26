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

    /// Parsing the data failed
    #[oai(status = 400)]
    Malformed,

    /// Validating the game failed
    #[oai(status = 418)]
    InvalidGame,

    /// The run already exists
    #[oai(status = 409)]
    AlreadyExists,
}

#[derive(Debug, Clone, Object)]
pub struct DataWrapper {
    pub data: Vec<serde_json::Value>,
}

#[derive(ApiResponse)]
pub enum GetDataResponse {
    /// Return requested games
    #[oai(status = 200)]
    Ok(Json<DataWrapper>),
}
