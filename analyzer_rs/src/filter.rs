use oispa_halla_analytics::server::api::internal_types::Game;

pub fn filter(data: Vec<Game>, allow_abandoned: bool, only_abandoned: bool) -> Vec<Game> {
    return data.into_iter().filter(|i| true).collect();
}
