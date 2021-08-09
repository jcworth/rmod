use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use crate::{error::RmError, recursive, Config, NodeModuleMap};

pub fn init_search(
    is_searching: Arc<AtomicBool>,
    config: &Config,
) -> JoinHandle<Result<NodeModuleMap, RmError>> {
    // Run search & count in separate thread
    let target_dir = config.target_dir.clone();
    thread::spawn(move || -> Result<NodeModuleMap, RmError> {
        let mut node_map = NodeModuleMap::new();

        // search and on ok count
        match recursive::recursive_search(Path::new(&target_dir), &mut node_map) {
            Ok(_) => {
                // Store size of node folders
                for dir in &mut node_map.dirs {
                    let new_file_size = recursive::recursive_count(dir.0).unwrap();
                    // bytes to mb
                    *dir.1 += new_file_size / 1000 / 1000;
                }

                is_searching.store(false, Ordering::Relaxed);
                Ok(node_map)
            }
            Err(_) => return Err(RmError::Io),
        }
    })
}
