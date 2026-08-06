use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use args::Args;
use clap::Parser;
use indexmap::IndexMap;
use loago::Tasks;

mod args;

const APP_NAME: &str = "loago";
const DATA_FILE_NAME: &str = "loago.json";
const EMPTY_JSON_FILE_CONTENT: &str = "{}";

fn main() -> Result<()> {
    let Args { action } = Args::parse();
    let data_dir = app_data_dir()?;
    let task_path = data_dir.join(DATA_FILE_NAME);
    let task_contents = read(&task_path)?.unwrap_or_else(|| String::from(EMPTY_JSON_FILE_CONTENT));
    let task_data: IndexMap<String, String> = serde_json::from_str(&task_contents)?;
    let tasks = Tasks::try_from(task_data)?;
    let new_tasks = action.execute(tasks.clone())?;
    if tasks != new_tasks {
        fs::create_dir_all(&data_dir)?;
        save_tasks(task_path, new_tasks)?;
    }
    Ok(())
}

fn app_data_dir() -> Result<PathBuf> {
    Ok(dirs::data_local_dir()
        .ok_or_else(|| anyhow!("local data directory wasn't found"))?
        .join(APP_NAME))
}

fn read(path: &Path) -> Result<Option<String>, io::Error> {
    use std::io::ErrorKind::*;
    let mut file = match OpenOptions::new()
        .read(true)
        .open(path)
    {
        Ok(file) => file,
        Err(err) if err.kind() == NotFound => return Ok(None),
        Err(err) => return Err(err),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(Some(contents))
}

fn save_tasks(path: impl AsRef<Path>, tasks: Tasks) -> Result<()> {
    let map: IndexMap<String, String> = tasks.into();
    let json = serde_json::to_string_pretty(&map)?;
    let mut data_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    data_file.write_all(json.as_bytes())?;
    Ok(())
}
