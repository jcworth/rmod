use rug::Float;
use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

mod recursive;
mod spinner;

pub struct Config {
    target_dir: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 2 {
            return Err("Incorrect number of args");
        }

        let target_dir = args[1].clone();

        Ok(Config { target_dir })
    }
}

enum FolderType {
    NodeModules
}

// Run the program
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Check target_dir
    is_directory_valid(&config.target_dir)?;

    // Shared running state
    let is_searching = Arc::new(AtomicBool::new(true));
    let is_searching_shared = is_searching.clone();

    // Create spinner in separate thread
    let spinner_handle = spinner::create_spinner(is_searching);

    // Run search & count in separate thread
    let search_handle = thread::spawn(move || -> thread::Result<NodeModuleMap> {
        let mut node_map = NodeModuleMap::new();

        // Search for node_modules
        if let Err(e) = recursive::recursive_search(Path::new(&config.target_dir), &mut node_map) {
            eprintln!("error: {:?}", e);
            process::exit(1);
        }

        // Store size of node folders
        for dir in &mut node_map.dirs {
            // bytes to mb
            let new_file_size = recursive::recursive_count(&dir.0).unwrap() / 1000 / 1000;
            *dir.1 += new_file_size;
        }

        is_searching_shared.store(false, Ordering::Relaxed);
        Ok(node_map)
    });

    // Wait for threads
    spinner_handle.join().unwrap();
    if let Ok(nm) = search_handle.join().unwrap() {
        for e in nm.dirs {
            println!("Directory: {:?}, Size on disk: {:?} ", e.0, e.1);
        }
    }

    // @TODO: Option to remove

    Ok(())
}

fn is_directory_valid(dir_name: &str) -> Result<(), &str> {
    if !Path::new(dir_name).exists() {
        return Err("Directory invalid. Try providing a relative or absolute path");
    }
    Ok(())
}

#[derive(Debug)]
pub struct NodeModuleMap {
    folder_count: u32,
    dirs: HashMap<PathBuf, Float>,
}

impl NodeModuleMap {
    fn new() -> NodeModuleMap {
        NodeModuleMap {
            dirs: HashMap::new(),
            folder_count: 0,
        }
    }

    fn add(&mut self, entry: PathBuf) -> () {
        self.dirs.insert(entry, Float::with_val(32, 0.0));
        self.folder_count += 1;
    }
}

enum FileSize {
    B,
    KB,
    MB,
    GB
}