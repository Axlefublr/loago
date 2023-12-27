use std::path::PathBuf;
use loago::APP_NAME;

mod args;

pub const DATA_FILE_NAME: &str = "loago.json";

fn main() {
    println!("Hello, world!");
}

pub fn default_path() -> Result<PathBuf, &'static str> {
    let data_dir = dirs::data_local_dir().ok_or("local data directory wasn't found")?;
    Ok(data_dir.join(APP_NAME).join(DATA_FILE_NAME))
}

// pub fn ensure_exists() {
//     fs::create_dir_all(path)
// }