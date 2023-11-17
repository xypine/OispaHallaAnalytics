use crate::Game;
use rayon::prelude::*;
use twothousand_forty_eight::unified::validate;

use crate::{io, libproxy::reconstruct_history};

pub fn filter(data: Vec<Game>, allow_abandoned: bool, only_abandoned: bool) -> Vec<Game> {
    data.par_iter()
        .filter(|g| validate(&g.data_raw).is_ok())
        .cloned()
        .collect::<Vec<_>>()
}

pub fn get_filtered_data() -> Vec<Game> {
    println!("Loading games...");
    let raw_data = io::load_folder("./data");
    println!("De-duplicating games...");
    let mut data = io::merge_data(raw_data).data;
    println!("Found {} games...", data.len());
    println!("Filtering data...");
    data = filter(data, true, false);
    println!("Filtered to {} games...", data.len());
    println!();
    data
}
