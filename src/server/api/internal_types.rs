use serde::{Serialize, Deserialize};
use twothousand_forty_eight::recording::Recording;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ParsedGame {
    pub recording: Recording,
    pub won: bool,
    pub abandoned: bool,
    pub score: usize,
    pub computed_score: usize,
    pub computed_score_margin: usize,
    pub timestamp_ms: usize
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Game {
    pub id: i64,
    pub data_raw: String,
    // pub data_parsed: String,
    pub hash: String
}