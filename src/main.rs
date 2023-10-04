use std::{env, process};

use minigrep::config::{Config, EnvVars};

fn main() {
    let args = env::args().collect();
    let config = Config::build(args, EnvVars::fetch()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
