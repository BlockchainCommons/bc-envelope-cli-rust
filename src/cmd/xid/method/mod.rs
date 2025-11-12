pub mod add;
pub mod all;
pub mod at;
pub mod count;
pub mod remove;

use anyhow::Result;
use clap::{Args, Subcommand};

/// Work a XID document's resolution methods.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Add(add::CommandArgs),
    All(all::CommandArgs),
    At(at::CommandArgs),
    Count(count::CommandArgs),
    Remove(remove::CommandArgs),
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::Add(args) => args.exec(),
            SubCommands::All(args) => args.exec(),
            SubCommands::At(args) => args.exec(),
            SubCommands::Count(args) => args.exec(),
            SubCommands::Remove(args) => args.exec(),
        }
    }
}
