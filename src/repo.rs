use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

pub struct JsonRepo(File);

impl JsonRepo {
    pub fn with_read(file: &Path) -> io::Result<Self> {
        Ok(JsonRepo(OpenOptions::new().read(true).open(file)?))
    }

    pub fn with_read_write(file: &Path) -> io::Result<Self> {
        Ok(JsonRepo(OpenOptions::new().read(true).write(true).open(file)?))
    }
}