use array_tool::vec::Union;
use csv::Writer;
use serde::{Deserialize, Serialize};

use crate::Game;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{self, PathBuf};

pub fn load_file(path: PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    Ok(data)
}

pub struct DataWrapper {
    pub data: Data,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Data {
    pub data: Vec<Game>,
}

pub fn load_and_parse(path: PathBuf) -> Option<Data> {
    let contents = load_file(path.clone());
    match contents {
        Err(err) => {
            println!(
                "WARNING: Failed to open file {}, because of error: {}",
                path.display(),
                err
            );
            None
        }
        Ok(to_parse) => {
            let parse_result: Result<Data, serde_json::Error> = serde_json::from_str(&to_parse);

            match parse_result {
                Err(err) => {
                    println!(
                        "WARNING: Failed to parse file {}, because of error: {}",
                        path.display(),
                        err
                    );
                    None
                }
                Ok(rec) => Some(rec),
            }
        }
    }
}

pub fn load_folder(folder_path: &str) -> Vec<Data> {
    let mut data = vec![];

    let paths = std::fs::read_dir(folder_path).expect("Failed to scan folder for files");

    let mut valid_paths = vec![];
    for i in paths {
        match i {
            Err(_) => {} // Don't care, skip the file
            Ok(f) => {
                match f.path().extension() {
                    None => {} // Don't care, skip
                    Some(extension) => {
                        if extension == "json" {
                            valid_paths.push(f.path());
                        }
                    }
                }
            }
        }
    }
    println!("Discovered {} potential data files!", valid_paths.len());

    println!("Parsing data...");
    for path in valid_paths {
        let data_result = load_and_parse(path.clone());
        if let Some(d) = data_result {
            println!("Loaded {} games from {}...", d.data.len(), path.display());
            data.push(d);
        }
    }

    data
}

pub fn merge_data(data: Vec<Data>) -> Data {
    let count_before = data.iter().map(|i| i.data.len()).sum::<usize>();
    let mut composite: Vec<Game> = vec![];
    for d in data {
        composite = composite.union(d.data);
    }
    let count_after = composite.len();
    println!("Removed {} duplicates...", count_before - count_after);
    Data { data: composite }
}

pub fn write_csv<T: std::convert::AsRef<[u8]>, I: IntoIterator<Item = T>>(
    path: &str,
    to_write: Vec<I>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Writing {}...", path);
    let mut wtr = Writer::from_path(path)?;
    for i in to_write {
        wtr.write_record(i)?;
    }
    wtr.flush()?;
    println!();
    Ok(())
}

pub fn write_composite(games: &Vec<Game>, path: &str) {
    println!("Writing {}...", path);
    let mut file = File::create(path).unwrap();
    for g in games {
        file.write_all(g.data_raw.as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }
}
