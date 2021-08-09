use rm_modules::{init, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    match init::run(config) {
        Ok(_) => println!("nice"),
        Err(e) => {
            if let Some(e) = e.downcast_ref::<std::io::Error>() {
                eprintln!("File IO error: {:?}", e);
                process::exit(1);
            } else {
                eprintln!("Application Error: {:?}", e);
                process::exit(1);
            }
        }
    };
}
