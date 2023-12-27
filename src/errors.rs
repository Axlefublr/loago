use std::error::Error;
use std::fmt;

pub trait AsErrStr {
    fn as_str(&self) -> &'static str;
}

#[derive(Debug)]
pub struct DataDirNotFoundError;

impl AsErrStr for DataDirNotFoundError {
    fn as_str(&self) -> &'static str {
        "local data directory wasn't found"
    }
}

impl Error for DataDirNotFoundError {}

impl fmt::Display for DataDirNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
