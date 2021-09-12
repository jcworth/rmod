use crate::error::RmError;

use std::{fs, path::Path};

pub fn remove_folders<T: AsRef<Path>>(map: Vec<T>) -> Result<(), RmError> {
    for f in &map {
        fs::remove_dir_all(f)?;
    }
    Ok(())
}
