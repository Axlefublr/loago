use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use clap::Parser;
use clap::Subcommand;
use loago::Tasks;

const HOURS_IN_DAY: i64 = 24;
const MINUTES_IN_HOUR: i64 = 60;

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
    #[command(visible_alias = "update")]
    #[command(visible_alias = "reset")]
    Do { tasks: Vec<String> },
    /// View all (default) or specified tasks, with how many days (and
    /// optionally, hours and minutes) ago you last did them.
    #[command(visible_alias = "list")]
    #[command(visible_alias = "look")]
    #[command(visible_alias = "see")]
    View {
        /// Show hours and minutes too, in this format: `{days}d {hours}h
        /// {minutes}m`
        #[arg(short, long)]
        minutes: bool,
        tasks:   Option<Vec<String>>,
    },
    /// Remove specified tasks from the list.
    #[command(visible_alias = "delete")]
    Remove { tasks: Vec<String> },
}

impl Action {
    pub fn execute(
        self,
        path: impl AsRef<Path>,
        mut tasks: Tasks,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Do { tasks: provided } => {
                tasks.update_multiple(provided);
                save(tasks, path)
            },
            Self::Remove { tasks: provided } => {
                tasks.remove_multiple(&provided);
                save(tasks, path)
            },
            Self::View {
                minutes,
                tasks: provided,
            } => {
                if let Some(provided) = provided {
                    tasks.keep_multiple(provided);
                }
                if minutes {
                    print!(
                        "{}",
                        tasks.output(|timestamp| {
                            let days = timestamp.num_days();
                            let total_hours = timestamp.num_hours();
                            let total_minutes = timestamp.num_minutes();
                            let hours = total_hours - (days * HOURS_IN_DAY);
                            let minutes =
                                total_minutes - (total_hours * MINUTES_IN_HOUR);
                            format!("{days}d {hours}h {minutes}m")
                        })
                    )
                } else {
                    print!("{}", tasks.output_days());
                }
                Ok(())
            },
        }
    }
}

fn save(tasks: Tasks, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let map: HashMap<String, String> = tasks.into();
    let json = serde_json::to_string_pretty(&map)?;
    let mut data_file =
        OpenOptions::new().write(true).truncate(true).open(path)?;
    data_file.write_all(json.as_bytes())?;
    Ok(())
}
