use rug::Float;

use crate::NodeModuleMap;

use std::fs;

pub fn remove_folders(module_map: NodeModuleMap) -> std::io::Result<Float> {
		let mut total_space = Float::with_val(32, 0.0);
    for file in &module_map.dirs {
				total_space += file.1;
        fs::remove_dir_all(file.0)?;
    }
    Ok(total_space)
}
