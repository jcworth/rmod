use std::path::Path;

pub fn is_directory_valid(dir_name: &str) -> Result<(), &str> {
    if !Path::new(dir_name).exists() {
        return Err("Directory invalid. Try providing a relative or absolute path");
    }
    Ok(())
}
