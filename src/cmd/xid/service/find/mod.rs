pub mod name;
pub mod uri;

use anyhow::Result;
use clap::{Args, Subcommand};

/// Find all XID services matching the given criteria
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Uri(uri::CommandArgs),
    Name(name::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::Uri(args) => args.exec(),
            SubCommands::Name(args) => args.exec(),
        }
    }
}
