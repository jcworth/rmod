use std::os::macos::fs::MetadataExt;

use crate::{
    error::RmError,
    spinner::{SpinStyle, Spinner},
    utils,
    walk::{EntryOptions, EntryWalk},
    Config, FileSize, NodeModuleMap,
};

// Run the program
pub fn init(config: Config) -> Result<NodeModuleMap, RmError> {
    // Check target_dir
    utils::is_directory_valid(&config.target_dir)?;

    let walker_options = EntryOptions::new(true, true, false);
    let walker = EntryWalk::new(config.target_dir.into(), walker_options)?;
    let mut nm_map = NodeModuleMap::new();

    let spinner = Spinner::new();

    spinner.set_style(SpinStyle::Search);
    let entries = walker
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(|v| v.is_node_modules())
        .collect::<Vec<_>>();

    // Set spinner style
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

            // TODO: Print path name
            spinner.msg((e.path().to_str().unwrap(), size));
        }

        nm_map.add(e.path().to_path_buf(), FileSize::MB.get_value(size));
    }

    spinner.end();
    Ok(nm_map)
}
