use std::os::macos::fs::MetadataExt;

use crate::{
    error::RmError,
    spinner::{SpinStyle, Spinner},
    utils,
    walk::{EntryOptions, EntryWalk},
    Config, FileSize, NodeModuleMap,
};

// Run the program
pub fn init(config: Config) -> Result<f64, RmError> {
    // Check target_dir
    utils::is_directory_valid(&config.target_dir)?;
		
    let mut nm_map = NodeModuleMap::new();

    let walker_options = EntryOptions::new(true, true, false);
    let walker = EntryWalk::new(config.target_dir.into(), walker_options)?;

    let spinner = Spinner::new();

    spinner.set_style(SpinStyle::Search);
    let entries = walker
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(|v| v.is_node_modules())
        .collect::<Vec<_>>();

		// Return if node_modules not found
		if entries.is_empty() {
			return Err(RmError::NotFound)
		}

    spinner.set_style(SpinStyle::Count);
    //walk through each dir, total size, add it to nm_map
    let nm_walker_options = EntryOptions::new(false, true, true);
    for e in &entries {
        let mut size: f64 = 0.0;

        for f in EntryWalk::new(e.path().to_path_buf(), nm_walker_options)?
            .into_iter()
            .filter_map(|v| v.ok())
        {
            let f_size = f.meta().st_blocks() * 512;
            size += f_size as f64;

            if let Some(p) = e.path().to_str() {
                spinner.msg((p, size));
            }
        }

        nm_map.add(e.path().to_path_buf(), FileSize::MB.get_value(size));
    }

    // Calculate total size
    let tsize = nm_map.dirs.iter().map(|v| v.1).sum::<f64>();
    spinner.end();

		// TODO: Prompt for confirmation
		
    Ok(tsize)
}
