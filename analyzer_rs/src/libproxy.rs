use memoize::memoize;
use twothousand_forty_eight::{
    direction::Direction,
    unified::{
        game::GameState,
        reconstruction::{HistoryReconstruction, Reconstructable},
        ParseResult,
    },
    v1::{
        parser::ParseError,
        recording::Recording,
        validator::{self, ValidationError},
    },
};

pub fn parse_data(data: String) -> Result<ParseResult, String> {
    twothousand_forty_eight::unified::parse(&data).map_err(|e| e.to_string())
}

pub fn reconstruct_history(data: String) -> Result<HistoryReconstruction, String> {
    twothousand_forty_eight::unified::reconstruct(&data).map_err(|e| e.to_string())
}

pub fn get_gamestate(data: String) -> Result<GameState, String> {
    twothousand_forty_eight::unified::get_gamestate(&data).map_err(|e| e.to_string())
}

pub fn get_moves(rec: &ParseResult) -> Vec<Direction> {
    match rec {
        ParseResult::V1(rec) => rec.history.iter().map(|i| i.1).collect(),
        ParseResult::V2(rec) => rec.moves.clone(),
    }
}
