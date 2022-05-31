mod io;
mod filter;
mod analyzer;

fn main() {
    println!("Loading games...");
    let raw_data = io::load_folder("./data");
    println!("De-duplicating games...");
    let mut data = io::merge_data(raw_data).data;
    println!("Found {} games...\n", data.len());

    println!("Filtering data...");
    data = filter::filter(data, true, false);
    println!("Filtered to {} games...", data.len());
    println!("Analyzing data...");
    analyzer::analyze_general(data.clone(), "out_all.csv");
    analyzer::analyze_frequence_move_first(data.clone(), "out_frequency_move_first.csv");
    analyzer::analyze_frequence_moves(data.clone(), "out_frequency_moves.csv");
    analyzer::analyze_first_move_vs_score(data.clone(), "out_first_move_vs_score.csv");

    println!("Filtering data to only finished games...");
    let data_completed = filter::filter(data.clone(), false, false);
    println!("Filtered to {} games...", data_completed.len());
    println!("Analyzing data...");
    analyzer::analyze_general(data_completed, "out_completed.csv");

    println!("Filtering data to only abandoned games...");
    let data_abandoned = filter::filter(data.clone(), true, true);
    println!("Filtered to {} games...", data_abandoned.len());
    println!("Analyzing data...");
    analyzer::analyze_general(data_abandoned, "out_abandoned.csv");
}
