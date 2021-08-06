use std::{
    error::Error,
    sync::{atomic::AtomicBool, Arc},
};

use crate::{remove, search, spinner, utils, Config};

// Run the program
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Check target_dir
    utils::is_directory_valid(&config.target_dir)?;

    // Shared running state
    let is_searching = Arc::new(AtomicBool::new(true));
    let is_searching_shared = is_searching.clone();

    // Create spinner & begin search in separate threads
    let spinner_handle = spinner::create_spinner(is_searching);
    let init_search_handle = search::init_search(is_searching_shared, &config);

    // Wait for threads
    spinner_handle.join().unwrap();
    if let Ok(nm) = init_search_handle.join().unwrap() {
        for e in &nm.dirs {
            println!("Directory: {:?}, Size on disk: {:?} ", e.0, e.1);
        }
        // @TODO: this should be behind a y/n input
        if let Ok(free_space) = remove::remove_folders(nm) {
            println!("Space freed: {:?}", &free_space);
        }
    }
    Ok(())
}
