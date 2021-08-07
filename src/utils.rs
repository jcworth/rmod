use std::{ffi::OsStr, fs::DirEntry, path::Path};

pub fn is_directory_valid(dir_name: &str) -> Result<(), &str> {
    if !Path::new(dir_name).exists() {
        return Err("Directory invalid. Try providing a relative or absolute path");
    }
    Ok(())
}

// Return bool if folder hidden/not
pub fn is_hidden(entry: &DirEntry) -> bool {
	entry.file_name().to_string_lossy().starts_with('.')
}

// Return bool if folder is named node_modules
pub fn is_node_modules(file: &Path) -> bool {
	file.file_name() == Some(OsStr::new("node_modules"))
}
