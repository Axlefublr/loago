use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use args::Args;
use clap::Parser;
use loago::Tasks;

mod args;

const APP_NAME: &str = "loago";
const DATA_FILE_NAME: &str = "loago.json";
const EMPTY_JSON_FILE_CONTENT: &[u8; 2] = b"{}";

fn main() -> Result<(), Box<dyn Error>> {
    let Args { action } = Args::parse();
    let data_dir = app_data_dir()?;
    let path = ensure_exists(data_dir, DATA_FILE_NAME)?;
    let contents = read(&path)?;
    let data: HashMap<String, String> = serde_json::from_str(&contents)?;
    let tasks = Tasks::try_from(data)?;
    action.execute(
        path,
        tasks,
    )?;
    Ok(())
}

fn app_data_dir() -> Result<PathBuf, &'static str> {
    Ok(dirs::data_local_dir()
        .ok_or("local data directory wasn't found")?
        .join(APP_NAME))
}

fn ensure_exists(
    data_dir: PathBuf,
    data_file: impl AsRef<Path>,
) -> Result<PathBuf, io::Error> {
    fs::create_dir_all(&data_dir)?;
    let full_path = data_dir.join(data_file);
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&full_path)
    {
        Ok(mut file) => {
            file.write_all(EMPTY_JSON_FILE_CONTENT)?;
            file.flush()?;
        },
        Err(error) => {
            use std::io::ErrorKind::*;
            if let AlreadyExists = error.kind() {
            } else {
                return Err(error);
            }
        },
    };
    Ok(full_path)
}

fn read(path: &Path) -> Result<String, io::Error> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
