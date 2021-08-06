use std::{
    path::Path,
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use crate::{recursive, Config, NodeModuleMap};

pub fn init_search(
    is_searching: Arc<AtomicBool>,
    config: &Config,
) -> JoinHandle<thread::Result<NodeModuleMap>> {
    // Run search & count in separate thread
    let target_dir = config.target_dir.clone();
    let search_handle = thread::spawn(move || -> thread::Result<NodeModuleMap> {
        let mut node_map = NodeModuleMap::new();

        // Search for node_modules
        if let Err(e) = recursive::recursive_search(Path::new(&target_dir), &mut node_map) {
            // @TODO: this error handling feels wrong
            eprintln!("error: {:?}", e);
            process::exit(1);
        }

        // Store size of node folders
        for dir in &mut node_map.dirs {
            // bytes to mb
            let new_file_size = recursive::recursive_count(&dir.0).unwrap() / 1000 / 1000;
            *dir.1 += new_file_size;
        }

        is_searching.store(false, Ordering::Relaxed);
        Ok(node_map)
    });
    search_handle
}
