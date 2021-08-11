use rm_modules::{error::RmError, init, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    match init::run(config) {
        Ok(result) => println!("Removed {}mb", result),
        Err(e) => {
            match e {
                RmError::Io => eprintln!("{}", e),
                RmError::InvalidDir => eprintln!("{}", e),
                _ => eprintln!("Unknown error"),
            }
            process::exit(1);
        }
    };
}
