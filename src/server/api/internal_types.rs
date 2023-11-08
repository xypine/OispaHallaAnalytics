use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Game {
    pub data_raw: String,
    pub hash: String,
    pub client: Option<String>,
    pub created_at: i64,
}
