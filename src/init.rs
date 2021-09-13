use std::{io::{self, BufRead}, os::macos::fs::MetadataExt};

use crate::{
    error::RmError,
    remove::remove_folders,
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

    // Initialise dir walker options
    let walker_options = EntryOptions::new(true, true, false);
    let walker = EntryWalk::new(config.target().into(), walker_options)?;

    let spinner = Spinner::new();
    spinner.set_style(SpinStyle::Search);

    let entries = walker
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(|v| v.is_node_modules())
        .collect::<Vec<_>>();

    // Return if node_modules not found
    if entries.is_empty() {
        return Err(RmError::NotFound);
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

    nm_map.total_size = nm_map.dirs.iter().map(|v| v.1).sum::<f64>();

    spinner.end();

    eprintln!("Found the following folders:");
    for (e, f) in nm_map.dirs.iter() {
        eprintln!("{:?}, {:.2} MB", e, f);
    }

    eprintln!("Total size: {:.2} MB, delete all? (y/n)", nm_map.total_size());

    // stdin buffer & lock - faster when locked
    let mut str_buf = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    loop {
        stdin_handle.read_line(&mut str_buf)?;

        match str_buf.trim() {
            "y" => {
                remove_folders(nm_map.dirs.iter().map(|v| v.0).collect::<Vec<_>>())?;
                break;
            }
            "n" => utils::exit(1),
            _ => {
                eprintln!("Invalid input, try 'y' or 'n'");
                str_buf.clear();
            }
        }
    }

    Ok(nm_map.total_size())
}
