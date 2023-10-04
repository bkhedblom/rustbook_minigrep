use std::{error::Error, fs};

use config::*;
use search::*;

pub mod config;
mod search;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents, config.ignore_case) {
        println!("{line}");
    }
    Ok(())
}
