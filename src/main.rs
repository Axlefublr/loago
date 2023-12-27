use std::path::PathBuf;
use loago::APP_NAME;
use loago::errors::AsErrStr;

mod args;

pub const DATA_FILE_NAME: &str = "loago.json";

fn main() {
    println!("Hello, world!");
}

pub trait UserFacing<T> {
    fn to_friendly(self) -> Result<T, &'static str>;
}

impl<T, E: AsErrStr> UserFacing<T> for Result<T, E> {
    fn to_friendly(self) -> Result<T, &'static str> {
        self.map_err(|err| err.as_str())
    }
}

pub fn default_path() -> Result<PathBuf, &'static str> {
    let data_dir = dirs::data_local_dir().ok_or("local data directory wasn't found")?;
    Ok(data_dir.join(APP_NAME).join(DATA_FILE_NAME))
}

// pub fn ensure_exists() {
//     fs::create_dir_all(path)
// }