use std::{ffi::OsStr, fs::DirEntry, path::Path, process};

use crate::error::RmError;

pub fn is_directory_valid(dir_name: &str) -> Result<(), RmError> {
    if Path::new(dir_name).exists() {
        Ok(())
    } else {
        Err(RmError::InvalidDir)
    }
}

// Return bool if folder hidden/not
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().starts_with('.')
}

// Return bool if folder is named node_modules
pub fn is_node_modules(file: &Path) -> bool {
    file.file_name().unwrap() == OsStr::new("node_modules") && file.is_dir()
}

pub fn exit(code: i32) {
    eprintln!("Exiting");
    process::exit(code);
}
