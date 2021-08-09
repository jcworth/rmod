use std::sync::{atomic::AtomicBool, Arc};

use crate::{error::RmError, search, spinner, utils, Config};

// Run the program
pub fn run(config: Config) -> Result<(), RmError> {
    // Check target_dir
    match utils::is_directory_valid(&config.target_dir) {
        Ok(_) => {
            // Shared running state
            let is_searching = Arc::new(AtomicBool::new(true));
            let is_searching_shared = is_searching.clone();

            // Create spinner & begin search in separate threads
            let spinner_handle = spinner::init_spinner(is_searching);
            let init_search_handle = search::init_search(is_searching_shared, &config);

            // Wait for threads
            // @TODO:  Better error handling here
            spinner_handle.join().unwrap();
            if let Err(e) = init_search_handle.join().unwrap() {
                return Err(e);
            };

            Ok(())
        }
        Err(e) => return Err(e),
    }
}
