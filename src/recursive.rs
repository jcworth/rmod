use std::{
    fs::{self, DirEntry, ReadDir},
    os::macos::fs::MetadataExt,
    path::{Path, PathBuf},
};

use rug::Float;

use crate::{error::RmError, utils, NodeModuleMap};

#[derive(Debug)]
pub struct Recursive {
    dir: ReadDir,
    store: NodeModuleMap
}

impl Recursive {
    pub fn new(path: &str) -> Result<Self, RmError> {
        let p = fs::read_dir(path)?;

        Ok(Self { dir: p, store: NodeModuleMap::new() })
    }

    pub fn search(&self) -> Result<(), RmError> {
        let entries = self
            .dir
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
                    self.store.add(file_path_buf);
                } else if file_type.is_dir() {
                    Self::search(self)?;
                }
            }
        }
        Ok(())
    }

    pub fn count(&self) -> Result<Float, RmError> {
        // @TODO: make block calc platform generic - currently unix/macos
        let entries = self.dir.filter_map(Result::ok);
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

// pub fn recursive_search(dir: &Path, module_map: &mut NodeModuleMap) -> Result<(), RmError> {
//     let path = fs::read_dir(dir)?
//         .filter_map(Result::ok)
//         .filter(|e| !utils::is_hidden(e));

//     for entry in path {
//         let file_path_buf = entry.path();
//         if let Ok(attribs) = file_path_buf.metadata() {
//             let file_type = &attribs.file_type();

//             if file_type.is_symlink() {
//                 continue;
//             } else if file_type.is_dir() && utils::is_node_modules(&file_path_buf) {
//                 module_map.add(file_path_buf);
//             } else if file_type.is_dir() {
//                 recursive_search(&file_path_buf, module_map)?;
//             }
//         }
//     }
//     Ok(())
// }

// pub fn recursive_count(dir: &Path) -> Result<Float, RmError> {
//     // @TODO: make block calc platform generic - currently unix/macos
//     let path = fs::read_dir(dir)?.filter_map(Result::ok);
//     let mut total_size = Float::with_val(32, 0.0);

//     for entry in path {
//         let file_path_buf = entry.path();
//         if let Ok(attribs) = file_path_buf.metadata() {
//             let file_type = &attribs.file_type();

//             if file_type.is_symlink() {
//                 continue;
//             } else if file_type.is_dir() {
//                 total_size += recursive_count(&file_path_buf)?;
//             } else {
//                 total_size += Float::with_val(32, attribs.st_blocks() * 512);
//             }
//         }
//     }
//     Ok(total_size)
// }
