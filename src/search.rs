use std::path::Path;

use rug::Float;

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
            let mut all_dirs_size = Float::new(32);
            for dir in &mut nm_map.dirs {
                let dir_size = dir.1;
                *dir_size += r.count(dir.0).unwrap();
                all_dirs_size += dir_size.to_owned();
                // *dir.1 += &a;
            }
            // bytes to mb
            nm_map.total_size += all_dirs_size / 1000 / 1000;
            r.spinner.end();
            Ok(nm_map)
        }
        Err(_) => Err(RmError::Io),
    }
}
