use error::RmError;
use std::{collections::HashMap, path::PathBuf};

pub mod error;
pub mod init;
pub mod recursive;
pub mod remove;
pub mod spinner;
pub mod utils;

pub struct Config {
    target_dir: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, RmError> {
        if args.len() != 2 {
            return Err(RmError::Config);
        }

        let target_dir = args[1].clone();

        Ok(Config { target_dir })
    }
}

#[derive(Debug)]
pub struct NodeModuleMap {
    folder_count: u32,
    dirs: HashMap<PathBuf, f64>,
    pub total_size: f64,
}

impl NodeModuleMap {
    fn new() -> NodeModuleMap {
        NodeModuleMap {
            dirs: HashMap::new(),
            folder_count: 0,
            total_size: 0.0,
        }
    }

    fn add(&mut self, entry: PathBuf) {
        self.dirs.insert(entry, 0.0);
        self.folder_count += 1;
    }
}

#[allow(dead_code)]
enum FileSize {
    B,
    KB,
    MB,
    GB,
}

impl FileSize {
    fn get_value(&self, val: f64) -> f64 {
        match self {
            FileSize::B => val,
            FileSize::KB => val / 1000_f64,
            FileSize::MB => val / 1000_f64 / 1000_f64,
            FileSize::GB => val / 1000_f64 / 1000_f64 / 1000_f64,
        }
    }
}
