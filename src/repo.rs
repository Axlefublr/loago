use crate::errors::DataDirNotFoundError;
use crate::APP_NAME;
use crate::DATA_FILE_NAME;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub struct JsonRepo(File);

impl JsonRepo {
    pub fn with_read(file: &Path) -> io::Result<Self> {
        Ok(JsonRepo(OpenOptions::new().read(true).open(file)?))
    }

    pub fn with_read_write(file: &Path) -> io::Result<Self> {
        Ok(JsonRepo(OpenOptions::new().read(true).write(true).open(file)?))
    }
}

pub fn default_path() -> Result<PathBuf, DataDirNotFoundError> {
    let data_dir = dirs::data_local_dir().ok_or(DataDirNotFoundError)?;
    Ok(data_dir.join(APP_NAME).join(DATA_FILE_NAME))
}
