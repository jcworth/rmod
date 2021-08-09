
use std::fmt;

#[derive(Debug)]
pub enum RmError {
    Io,
    Config,
    InvalidDir
}

impl std::error::Error for RmError {}

impl fmt::Display for RmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self {
            RmError::Io => "File IO error",
            RmError::Config => "File IO error",
            RmError::InvalidDir => "File IO error",
        };
        write!(f, "{}", err_msg)
    }
}
