use rm_modules;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = rm_modules::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rm_modules::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    };
}
