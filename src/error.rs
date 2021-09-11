use std::fmt;

#[derive(Debug)]
pub enum RmError {
    Io,
    Config,
    InvalidDir,
    NotFound
}

impl std::error::Error for RmError {}

impl fmt::Display for RmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self {
            RmError::Io => "File IO error.",
            RmError::Config => "Error parsing arguments. Incorrect number of args.",
            RmError::InvalidDir => "Directory invalid. Try providing a relative or absolute path.",
            RmError::NotFound => "No node_modules folder found."
        };
        write!(f, "{}", err_msg)
    }
}

impl From<std::io::Error> for RmError {
    fn from(_: std::io::Error) -> Self {
        RmError::Io
    }
}
