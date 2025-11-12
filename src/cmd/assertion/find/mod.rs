pub mod object;
pub mod predicate;

use anyhow::Result;
use clap::{Args, Subcommand};

/// Find all assertions matching the given criteria.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Object(object::CommandArgs),
    Predicate(predicate::CommandArgs),
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::Object(args) => args.exec(),
            SubCommands::Predicate(args) => args.exec(),
        }
    }
}
