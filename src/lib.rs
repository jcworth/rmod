use error::RmError;
use std::{collections::HashMap, path::PathBuf};

pub mod error;
pub mod init;
pub mod remove;
pub mod spinner;
pub mod utils;
pub mod walk;

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

    pub fn target(&self) -> &str {
        &self.target_dir
    }
}

#[derive(Debug)]
pub struct NodeModuleMap {
    folder_count: u32,
    dirs: HashMap<PathBuf, f64>,
    total_size: f64,
}

impl NodeModuleMap {
    pub fn new() -> NodeModuleMap {
        NodeModuleMap {
            dirs: HashMap::new(),
            folder_count: 0,
            total_size: 0.0,
        }
    }

    pub fn add(&mut self, entry: PathBuf, size: f64) {
        self.dirs.insert(entry, size);
        self.folder_count += 1;
    }

    pub fn total_size(&self) -> f64 {
        self.total_size
    }
}

#[allow(dead_code)]
pub enum FileSize {
    B,
    KB,
    MB,
    GB,
}

impl FileSize {
    pub fn get_value(&self, val: f64) -> f64 {
        match self {
            FileSize::B => val,
            FileSize::KB => val / 1000_f64,
            FileSize::MB => val / 1000_f64 / 1000_f64,
            FileSize::GB => val / 1000_f64 / 1000_f64 / 1000_f64,
        }
    }
}
