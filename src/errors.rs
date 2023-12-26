use std::error::Error;
use std::fmt;

pub struct DataDirNotFoundError;

impl Error for DataDirNotFoundError {}

impl fmt::Display for DataDirNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "local data directory wasn't found")
    }
}

impl fmt::Debug for DataDirNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
