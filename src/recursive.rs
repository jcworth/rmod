use std::{fs, os::macos::fs::MetadataExt, path::Path};

use rug::Float;

use crate::{error::RmError, utils, NodeModuleMap};

#[derive(Debug)]
pub struct Recursive {
    pub dir: String,
    // store: NodeModuleMap,
}

impl Recursive {
    pub fn new(path: &str) -> Result<Self, RmError> {
        if fs::metadata(&path).is_ok() {
            Ok(Self {
                dir: path.to_string(),
            })
        } else {
            Err(RmError::InvalidDir)
        }
    }

    pub fn search(&self, path: &Path, nm_map: &mut NodeModuleMap) -> Result<(), RmError> {
        let entries = fs::read_dir(path)?
            .filter_map(Result::ok)
            .filter(|e| !utils::is_hidden(e));

        for entry in entries {
            let file_path_buf = entry.path();
            if let Ok(attribs) = file_path_buf.metadata() {
                let file_type = &attribs.file_type();

                if file_type.is_symlink() {
                    continue;
                } else if file_type.is_dir() && utils::is_node_modules(&file_path_buf) {
                    nm_map.add(file_path_buf)
                } else if file_type.is_dir() {
                    self.search(&file_path_buf, nm_map)?;
                }
            }
        }
        Ok(())
    }

    pub fn count(&self, path: &Path) -> Result<Float, RmError> {
        // @TODO: make block calc platform generic - currently unix/macos
        let entries = fs::read_dir(path)?.filter_map(Result::ok);
        let mut total_size = Float::with_val(32, 0.0);

        for entry in entries {
            let file_path_buf = entry.path();
            if let Ok(attribs) = file_path_buf.metadata() {
                let file_type = &attribs.file_type();

                if file_type.is_symlink() {
                    continue;
                } else if file_type.is_dir() {
                    total_size += self.count(&file_path_buf)?;
                } else {
                    total_size += Float::with_val(32, attribs.st_blocks() * 512);
                }
            }
        }
        Ok(total_size)
    }
}
