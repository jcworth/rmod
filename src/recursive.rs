use std::{
    error::Error,
    ffi::OsStr,
    fs::{self, DirEntry},
    os::macos::fs::MetadataExt,
    path::{Path, PathBuf},
};

use rug::Float;

use crate::NodeModuleMap;

pub fn recursive_search<'a, 'b>(
    dir: &'a Path,
    module_map: &'b mut NodeModuleMap,
) -> Result<(), Box<dyn Error>> {
    let path = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .filter(is_not_hidden);

    for entry in path {
        let file_path_buf = entry.path();
        if let Ok(attribs) = file_path_buf.metadata() {
            let file_type = &attribs.file_type();

            if file_type.is_symlink() {
                continue;
            } else if file_type.is_dir() && is_node_modules(&file_path_buf) {
                module_map.add(file_path_buf);
            } else if file_type.is_dir() {
                recursive_search(&file_path_buf, module_map)?;
            }
        }
    }
    Ok(())
}

pub fn recursive_count<'c, 'd>(dir: &'c PathBuf) -> Result<Float, Box<dyn Error>> {
    // @TODO: make block calc platform generic - currently unix
    let path = fs::read_dir(dir)?.filter_map(Result::ok);
    let mut total_size = Float::with_val(32, 0.0);

    for entry in path {
        let file_path_buf = entry.path();
        if let Ok(attribs) = file_path_buf.metadata() {
            let file_type = &attribs.file_type();

            if file_type.is_symlink() {
                continue;
            } else if file_type.is_dir() {
                total_size += recursive_count(&file_path_buf)?;
            } else {
                total_size += Float::with_val(32, attribs.st_blocks() * 512);
            }
        }
    }
    Ok(total_size)
}

// Return bool if folder hidden/not
fn is_not_hidden(entry: &DirEntry) -> bool {
    !entry.file_name().to_string_lossy().starts_with(".")
}

// Return bool if folder is named node_modules
fn is_node_modules(file: &PathBuf) -> bool {
    file.file_name() == Some(OsStr::new("node_modules"))
}
