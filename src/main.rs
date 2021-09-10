use sj::{Config, init};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    match init::init(config) {
        Ok(result) => println!("Removed {:.2} MB", result.total_size),
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    };
}
