use crate::{error::RmError, recursive::Recursive, utils, Config, NodeModuleMap};
use std::path::Path;

use rug::Float;

// Run the program
pub fn run(config: Config) -> Result<NodeModuleMap, RmError> {
    // Check target_dir
    utils::is_directory_valid(&config.target_dir)?;

    let r = Recursive::new(&config.target_dir)?;
    let mut nm_map = NodeModuleMap::new();

    // search and on ok count
    let path = Path::new(&r.dir);
    match r.search(path, &mut nm_map) {
        Ok(_) => {
            // Store size of node_module folders
            let mut all_dirs_size = Float::new(32);

            r.spinner.set_count_style();

            for dir in &mut nm_map.dirs {
                let mut dir_size = dir.1.clone();
                dir_size += r.count(dir.0)?;

                // Add individual dir_size to total
                all_dirs_size += dir_size;
                r.spinner.msg(
                    (&all_dirs_size / Float::with_val(32, 1000) / Float::with_val(32, 1000))
                        .to_string(),
                );
            }

            // bytes to mb on total val
            nm_map.total_size += all_dirs_size / 1000 / 1000;
            r.spinner.end();
            Ok(nm_map)
        }
        Err(_) => Err(RmError::Io),
    }
}
