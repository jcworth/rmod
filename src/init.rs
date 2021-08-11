use rug::Float;

use crate::{error::RmError, search, utils, Config};

// Run the program
pub fn run(config: Config) -> Result<Float, RmError> {
    // Check target_dir
    match utils::is_directory_valid(&config.target_dir) {
        Ok(_) => {
            // @TODO: Spinner start here
            // // Create spinner & begin search in separate threads
            // let spinner_handle = spinner::init_spinner(is_searching);
            match search::init_search(&config) {
                // @TODO: SPinner end here
                // @TODO: return results from search
                Ok(res) => Ok(res),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}
