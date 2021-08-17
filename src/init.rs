use crate::{error::RmError, utils, walk::Walk, Config, FileSize, NodeModuleMap};
use std::path::Path;

// Run the program
pub fn run(config: Config) -> Result<NodeModuleMap, RmError> {
    // Check target_dir
    utils::is_directory_valid(&config.target_dir)?;

    let r = Walk::new(&config.target_dir)?;
    let mut nm_map = NodeModuleMap::new();

    // search and on ok count
    let path = Path::new(&r.dir);
    match r.search(path, &mut nm_map) {
        Ok(_) => {
            // Store size of node_module folders
            let mut all_dirs_size = 0.0;
            r.spinner.set_count_style();

            for dir in &mut nm_map.dirs {
                let mut dir_size = *dir.1;
                dir_size += r.count(dir.0)?;

                // Add individual dir_size to total
                all_dirs_size += dir_size;
                r.spinner
                    .msg(format!("{:.2}", FileSize::MB.get_value(all_dirs_size)));
            }

            // bytes to mb on total val
            nm_map.total_size += FileSize::MB.get_value(all_dirs_size);
            r.spinner.end();

            Ok(nm_map)
        }
        Err(_) => Err(RmError::Io),
    }
}
