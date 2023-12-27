use loago::errors::AsErrStr;

mod args;

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