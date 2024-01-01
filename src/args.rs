use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    #[command(visible_alias = "add")]
    #[command(visible_alias = "new")]
    Do,
    #[command(visible_alias = "list")]
    #[command(visible_alias = "look")]
    View,
    #[command(visible_alias = "delete")]
    Remove,
}
