use rm_modules::{init, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = init::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    };
}
