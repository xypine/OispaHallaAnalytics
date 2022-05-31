use std::collections::HashMap;

use oispa_halla_analytics::server::api::internal_types::Game;
use twothousand_forty_eight::{recording::{Recording, History}, direction::Direction};

fn bool_to_r_str(boolean: bool) -> String {
    if boolean {
        String::from("TRUE")
    }
    else {
        String::from("FALSE")
    }
}

fn move_to_str(mv: Direction) -> String {
    return format!("{:?}", mv)
        .replace("UP", "Ylös")
        .replace("RIGHT", "Oikealle")
        .replace("DOWN", "Alas")
        .replace("LEFT", "Vasemmalle")
    ;
}

pub fn analyze_general(data: Vec<Game>, filename: &str) {
    let recordings: Vec<&Recording> = data.iter().map(|i| &i.recording).collect();
    let histories: Vec<&History> = recordings.into_iter().map(|i| &i.history).collect();

    let times: Vec<usize> = data.iter().map(|i| i.timestamp_ms).collect();
    let scores: Vec<usize> = data.iter().map(|i| i.computed_score).collect();
    let wins: Vec<bool> = data.iter().map(|i| i.won).collect();
    let abandoneds: Vec<Option<bool>> = data.iter().map(|i| i.abandoned).collect();

    let mut to_write = vec![
        vec!["time".to_string(), "game_length".to_string(), "score".to_string(), "won".to_string(), "abandoned".to_string(), "move_first".to_string(), "move_last".to_string()],
    ];

    let mut frame_index: usize = 0;
    for frames in histories {
        let directions: Vec<Direction> = frames.iter().map(|i| i.1).collect();

        let time = times[frame_index].to_string();
        let length = directions.len();
        let score = scores[frame_index].to_string();
        let won = wins[frame_index];
        let abandoned = match abandoneds[frame_index] {
            None => false,
            Some(a) => a,
        };
        let move_first = directions[0];
        let move_last = match directions[length - 1] {
            Direction::END => directions[length - 2],
            _ => directions[length - 1],
        };

        to_write.push(vec![
            time, length.to_string(), score, bool_to_r_str(won), bool_to_r_str(abandoned), move_to_str(move_first), move_to_str(move_last)
        ]);
        frame_index += 1;
    }
    crate::io::write_csv(filename, to_write).expect("Failed to write csv");
}

pub fn analyze_frequence_move_first(data: Vec<Game>, filename: &str) {
    let recordings: Vec<&Recording> = data.iter().map(|i| &i.recording).collect();
    let histories: Vec<&History> = recordings.into_iter().map(|i| &i.history).collect();

    let mut moves_first = vec![];
    for frames in histories {
        let directions: Vec<Direction> = frames.iter().map(|i| i.1).collect();
        let move_first = directions[0];
        moves_first.push(move_first);
    }

    let c = moves_first.iter().count();
    let c_up = moves_first.iter().filter(|m| m == &&Direction::UP).count();
    let c_right = moves_first.iter().filter(|m| m == &&Direction::RIGHT).count();
    let c_down = moves_first.iter().filter(|m| m == &&Direction::DOWN).count();
    let c_left = moves_first.iter().filter(|m| m == &&Direction::LEFT).count();
    let to_write = vec![
        vec!["Ensimmäinen siirto".to_string(), "f".to_string(), "f%".to_string()],
        vec!["Ylös".to_string(), c_up.to_string(), (c_up as f64 / c as f64).to_string()],
        vec!["Oikealle".to_string(), c_right.to_string(), (c_right as f64 / c as f64).to_string()],
        vec!["Alas".to_string(), c_down.to_string(), (c_down as f64 / c as f64).to_string()],
        vec!["Vasemmalle".to_string(), c_left.to_string(), (c_left as f64 / c as f64).to_string()],
        vec!["Yhteensä".to_string(), c.to_string(), (c as f64 / c as f64).to_string()],
    ];
    crate::io::write_csv(filename, to_write).expect("Failed to write csv");
}

pub fn analyze_frequence_moves(data: Vec<Game>, filename: &str) {
    let recordings: Vec<&Recording> = data.iter().map(|i| &i.recording).collect();
    let histories: Vec<&History> = recordings.into_iter().map(|i| &i.history).collect();

    let mut moves = vec![];
    for frames in histories {
        let mut directions: Vec<Direction> = frames.iter().map(|i| i.1).collect();
        moves.append(&mut directions);
    }

    let c = moves.iter().count();
    let c_up = moves.iter().filter(|m| m == &&Direction::UP).count();
    let c_right = moves.iter().filter(|m| m == &&Direction::RIGHT).count();
    let c_down = moves.iter().filter(|m| m == &&Direction::DOWN).count();
    let c_left = moves.iter().filter(|m| m == &&Direction::LEFT).count();
    let to_write = vec![
        vec!["Suunta".to_string(), "f".to_string(), "f_per".to_string()],
        vec!["Ylös".to_string(), c_up.to_string(), (c_up as f64 / c as f64).to_string()],
        vec!["Oikealle".to_string(), c_right.to_string(), (c_right as f64 / c as f64).to_string()],
        vec!["Alas".to_string(), c_down.to_string(), (c_down as f64 / c as f64).to_string()],
        vec!["Vasemmalle".to_string(), c_left.to_string(), (c_left as f64 / c as f64).to_string()],
        vec!["Yhteensä".to_string(), c.to_string(), (c as f64 / c as f64).to_string()],
    ];
    crate::io::write_csv(filename, to_write).expect("Failed to write csv");
}

pub fn analyze_first_move_vs_score(data: Vec<Game>, filename: &str) {
    let recordings: Vec<&Recording> = data.iter().map(|i| &i.recording).collect();
    let histories: Vec<&History> = recordings.into_iter().map(|i| &i.history).collect();

    let scores: Vec<usize> = data.iter().map(|i| i.computed_score).collect();

    let mut moves_first = vec![];
    let mut sum_scores:HashMap<Direction, usize> = HashMap::new();
    let mut frame_index: usize = 0;
    for frames in histories {
        let directions: Vec<Direction> = frames.iter().map(|i| i.1).collect();

        let move_first = directions[0];
        let score = scores[frame_index];
        match sum_scores.get_mut(&move_first) {
            Some( val ) => {
                *val = *val + score;
            },
            None => {
                sum_scores.insert(move_first, 1);
            },
        }
        moves_first.push(move_first);

        frame_index += 1;
    }

    let c_up = moves_first.iter().filter(|m| m == &&Direction::UP).count();
    let c_right = moves_first.iter().filter(|m| m == &&Direction::RIGHT).count();
    let c_down = moves_first.iter().filter(|m| m == &&Direction::DOWN).count();
    let c_left = moves_first.iter().filter(|m| m == &&Direction::LEFT).count();

    let s_up = *sum_scores.get(&Direction::UP).unwrap_or_else(|| &0);
    let s_right = *sum_scores.get(&Direction::RIGHT).unwrap_or_else(|| &0);
    let s_down = *sum_scores.get(&Direction::DOWN).unwrap_or_else(|| &0);
    let s_left = *sum_scores.get(&Direction::LEFT).unwrap_or_else(|| &0);
    let to_write = vec![
        vec!["Suunta".to_string(), "Pisteet".to_string(), "Pisteet_avg".to_string()],
        vec!["Ylös".to_string(), s_up.to_string(), (s_up as f64 / c_up as f64).to_string()],
        vec!["Oikealle".to_string(), s_right.to_string(), (s_right as f64 / c_right as f64).to_string()],
        vec!["Alas".to_string(), s_down.to_string(), (s_down as f64 / c_down as f64).to_string()],
        vec!["Vasemmalle".to_string(), s_left.to_string(), (s_left as f64 / c_left as f64).to_string()],
    ];
    crate::io::write_csv(filename, to_write).expect("Failed to write csv");
}