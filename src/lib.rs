use rug::Float;
use std::{collections::HashMap, path::PathBuf};

pub mod init;
pub mod recursive;
pub mod remove;
pub mod search;
pub mod spinner;
pub mod utils;
pub mod error;

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

    fn add(&mut self, entry: PathBuf) {
        self.dirs.insert(entry, Float::with_val(32, 0.0));
        self.folder_count += 1;
    }
}

// enum FolderType {
//     NodeModules,
// }

// enum FileSize {
//     B,
//     KB,
//     MB,
//     GB,
// }
