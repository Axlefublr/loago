use loago::APP_NAME;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

mod args;

const DATA_FILE_NAME: &str = "loago.json";
const EMPTY_JSON_FILE_CONTENT: &[u8; 2] = b"{}";

fn main() -> Result<(), Box<dyn Error>> {
    let data_dir = app_data_dir()?;
    ensure_exists(data_dir, DATA_FILE_NAME)?;
    Ok(())
}

fn app_data_dir() -> Result<PathBuf, &'static str> {
    Ok(dirs::data_local_dir()
        .ok_or("local data directory wasn't found")?
        .join(APP_NAME))
}

fn ensure_exists(data_dir: PathBuf, data_file: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&data_dir)?;
    let full_path = data_dir.join(data_file);
    match OpenOptions::new().write(true).create_new(true).open(full_path) {
        Ok(mut file) => {
            file.write_all(EMPTY_JSON_FILE_CONTENT)?;
            file.flush()?;
        }
        Err(error) => {
            use std::io::ErrorKind::*;
            if let AlreadyExists = error.kind() {
            } else {
                return Err(error);
            }
        }
    };
    Ok(())
}
