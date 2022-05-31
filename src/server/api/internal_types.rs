use serde::{Serialize, Deserialize};
use twothousand_forty_eight::recording::Recording;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Game {
    pub recording: Recording,
    pub won: bool,
    pub abandoned: Option<bool>,
    pub score: usize,
    pub computed_score: usize,
    pub computed_socre_margin: usize,
    pub timestamp_ms: usize
}