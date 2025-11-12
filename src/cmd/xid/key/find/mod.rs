pub mod inception;
pub mod name;
pub mod public;

use anyhow::Result;
use clap::{Args, Subcommand};

/// Find all XID keys matching the given criteria
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Inception(inception::CommandArgs),
    Name(name::CommandArgs),
    Public(public::CommandArgs),
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::Inception(args) => args.exec(),
            SubCommands::Name(args) => args.exec(),
            SubCommands::Public(args) => args.exec(),
        }
    }
}
