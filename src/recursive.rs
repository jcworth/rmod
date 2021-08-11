use std::{fs, os::macos::fs::MetadataExt};

use rug::Float;

use crate::{error::RmError, utils, NodeModuleMap};

#[derive(Debug)]
pub struct Recursive {
    dir: String,
    // store: NodeModuleMap,
}

impl Recursive {
    pub fn new(path: String) -> Result<Self, RmError> {
        if fs::metadata(&path).is_ok() {
            Ok(Self { dir: path })
        } else {
            Err(RmError::InvalidDir)
        }
    }

    // fn get_dir_iter(&self) -> Result<impl Iterator, RmError> {
    //     let dir = &self.dir;
    //     let dir_iter = fs::read_dir(dir)?.filter_map(Result::ok);

    //     Ok(dir_iter)
    // }

    pub fn search(&self, nm_map: &mut NodeModuleMap) -> Result<(), RmError> {
        let entries = fs::read_dir(&self.dir)?
            .filter_map(Result::ok)
            .filter(|e| !utils::is_hidden(e));

        for entry in entries {
            // println!("{:?}", entry)
            let file_path_buf = entry.path();
            if let Ok(attribs) = file_path_buf.metadata() {
                let file_type = &attribs.file_type();

                if file_type.is_symlink() {
                    continue;
                } else if file_type.is_dir() && utils::is_node_modules(&file_path_buf) {
                    nm_map.add(entry.path())
                } else if file_type.is_dir() {
                    Self::search(self, nm_map)?;
                }
            }
        }

        Ok(())
    }

    pub fn count(&self) -> Result<Float, RmError> {
        // @TODO: make block calc platform generic - currently unix/macos
        let entries = fs::read_dir(&self.dir)?.filter_map(Result::ok);
        let mut total_size = Float::with_val(32, 0.0);

        for entry in entries {
            let file_path_buf = entry.path();
            if let Ok(attribs) = file_path_buf.metadata() {
                let file_type = &attribs.file_type();

                if file_type.is_symlink() {
                    continue;
                } else if file_type.is_dir() {
                    total_size += Self::count(self)?;
                } else {
                    total_size += Float::with_val(32, attribs.st_blocks() * 512);
                }
            }
        }
        Ok(total_size)
    }
}
