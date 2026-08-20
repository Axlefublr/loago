use std::ops::Not;

use anyhow::Result;
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
    Do {
        #[arg(required = true)]
        tasks: Vec<String>,
    },
    /// View all (default) or specified tasks (if provided),
    /// with how many days (and optionally, hours and minutes) ago you last did them.
    #[command(visible_alias = "list")]
    #[command(visible_alias = "look")]
    #[command(visible_alias = "see")]
    View {
        /// Show hours and minutes too, in this format: `{days}d {hours}h
        /// {minutes}m`
        #[arg(short, long)]
        minutes: bool,
        /// Don't display these provided tasks.
        #[arg(short, long)]
        except: Vec<String>,
        tasks: Vec<String>,
    },
    /// Remove specified tasks from the list.
    #[command(visible_alias = "delete")]
    Remove {
        #[arg(required = true)]
        tasks: Vec<String>,
    },
}

impl Action {
    pub fn execute(self, mut tasks: Tasks) -> Result<Tasks> {
        match self {
            Self::Do { tasks: provided } => {
                tasks.update_multiple(provided);
            },
            Self::Remove { tasks: provided } => {
                tasks.remove_multiple(&provided);
            },
            Self::View {
                minutes,
                except,
                tasks: provided,
            } => {
                let mut ephemeral_tasks = tasks.clone();
                if provided.is_empty().not() {
                    ephemeral_tasks.keep_multiple(provided);
                }
                if except.is_empty().not() {
                    ephemeral_tasks.remove_multiple(&except);
                }
                if minutes {
                    print!(
                        "{}",
                        ephemeral_tasks.output(|timestamp| {
                            let days = timestamp.num_days();
                            let total_hours = timestamp.num_hours();
                            let total_minutes = timestamp.num_minutes();
                            let hours = total_hours - (days * HOURS_IN_DAY);
                            let minutes = total_minutes - (total_hours * MINUTES_IN_HOUR);
                            format!("{days}d {hours}h {minutes}m")
                        })
                    )
                } else {
                    print!("{}", ephemeral_tasks.output_days());
                }
            },
        }
        Ok(tasks)
    }
}
