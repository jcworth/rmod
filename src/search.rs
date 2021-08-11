use rug::Float;

use crate::{error::RmError, recursive::Recursive, Config, NodeModuleMap};

pub fn init_search(config: &Config) -> Result<Float, RmError> {
    // Run search & count in separate thread
    let target_dir = config.target_dir.clone();
    let r = Recursive::new(target_dir)?;
    let mut nm_map = NodeModuleMap::new();
    // search and on ok count
    match r.search(&mut nm_map) {
        Ok(_) => {
            // Store size of node folders
            for mut dir in nm_map.dirs {
                let new_file_size = r.count().unwrap();
                // bytes to mb
                dir.1 += new_file_size / 1000 / 1000;
            }

            Ok(nm_map.total_size)
        }
        Err(_) => Err(RmError::Io),
    }
}
