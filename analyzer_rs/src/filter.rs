use oispa_halla_analytics::server::api::internal_types::Game;

pub fn was_abandoned(g: &Game) -> bool {
    match g.abandoned {
        None => false,
        Some( a ) => a,
    }
}

pub fn filter(data: Vec<Game>, allow_abandoned: bool, only_abandoned: bool) -> Vec<Game> {
    return data.into_iter().filter(
        |i|
        i.recording.history.len() > 1 && (
            allow_abandoned || !was_abandoned(&i)
        ) && (
            !only_abandoned || was_abandoned(&i)
        )
    ).collect()
}