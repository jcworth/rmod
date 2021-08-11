use std::path::Path;

use crate::{error::RmError, recursive::Recursive, Config, NodeModuleMap};

pub fn init_search(config: &Config) -> Result<NodeModuleMap, RmError> {
    // Run search & count in separate thread
    // let target_dir = config.target_dir.clone();
    let r = Recursive::new(&config.target_dir)?;
    let mut nm_map = NodeModuleMap::new();
    // search and on ok count
    let path = Path::new(&r.dir);
    match r.search(path, &mut nm_map) {
        Ok(_) => {
            // Store size of node folders
            for dir in &mut nm_map.dirs {
                let new_file_size = r.count(dir.0).unwrap();
                // bytes to mb
                *dir.1 += new_file_size / 1000 / 1000;
            }

            Ok(nm_map)
        }
        Err(_) => Err(RmError::Io),
    }
}
