use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use clap::Parser;
use clap::Subcommand;
use loago::Tasks;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    /// Update tasks' dates to now.
    /// Creates tasks that didn't exist before.
    #[command(visible_alias = "add")]
    #[command(visible_alias = "new")]
    Do { tasks: Vec<String> },
    /// View all (default) or specified tasks, with how many days ago you last did them.
    #[command(visible_alias = "list")]
    #[command(visible_alias = "look")]
    View { tasks: Option<Vec<String>> },
    /// Remove specified tasks from the list.
    #[command(visible_alias = "delete")]
    Remove { tasks: Vec<String> },
}

impl Action {
    pub fn execute(self, data_file: File, mut tasks: Tasks) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Do { tasks: provided } => {
                tasks.update_multiple(provided);
                save(tasks, data_file)
            },
            Self::Remove { tasks: provided } => {
                tasks.remove_multiple(&provided);
                save(tasks, data_file)
            },
            Self::View { tasks: provided } => {
                if let Some(provided) = provided {
                    tasks.keep_multiple(provided);
                }
                print!("{}", tasks.output_days());
                Ok(())
            },
        }
    }
}

fn save(tasks: Tasks, mut data_file: File) -> Result<(), Box<dyn Error>> {
    let map: HashMap<String, String> = tasks.into();
    let json = serde_json::to_string_pretty(&map)?;
    data_file.write_all(json.as_bytes())?;
    Ok(())
}
