use std::path::Path;

use rug::Float;

use crate::{error::RmError, recursive::Recursive, Config, NodeModuleMap};

// Run search & count
pub fn init_search(config: &Config) -> Result<NodeModuleMap, RmError> {
    let r = Recursive::new(&config.target_dir)?;
    let mut nm_map = NodeModuleMap::new();

    // search and on ok count
    let path = Path::new(&r.dir);
    match r.search(path, &mut nm_map) {
        Ok(_) => {
            // Store size of node_module folders
            let mut all_dirs_size = Float::new(32);
            for dir in &mut nm_map.dirs {
                let mut dir_size = dir.1.clone();
                dir_size += r.count(dir.0).unwrap();

                // Add individual dir_size to total
                all_dirs_size += dir_size;
            }
            // bytes to mb on total val
            nm_map.total_size += all_dirs_size / 1000 / 1000;
            r.spinner.end();
            Ok(nm_map)
        }
        Err(_) => Err(RmError::Io),
    }
}
