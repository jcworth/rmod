use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use crate::{Config, NodeModuleMap, error::RmError, recursive::{self, Recursive}};

pub fn init_search(
    is_searching: Arc<AtomicBool>,
    config: &Config,
) -> Result<NodeModuleMap, RmError> {
    // Run search & count in separate thread
    let target_dir = config.target_dir.clone();
    let r = Recursive::new(&target_dir)?;
    // thread::spawn(move || -> Result<NodeModuleMap, RmError> {
        let mut node_map = NodeModuleMap::new();

        // search and on ok count
        match r.search() {
            Ok(_) => {
                // Store size of node folders
                for dir in &mut node_map.dirs {
                    let new_file_size = r.count().unwrap();
                    // bytes to mb
                    *dir.1 += new_file_size / 1000 / 1000;
                }

                is_searching.store(false, Ordering::Relaxed);
                Ok(node_map)
            }
            Err(_) => return Err(RmError::Io),
        }
    // })
}
