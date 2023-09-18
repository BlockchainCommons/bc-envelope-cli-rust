pub mod predicate;
pub mod object;

use clap::{Subcommand, Args};

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

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        match &self.command {
            SubCommands::Object(args) => args.exec(),
            SubCommands::Predicate(args) => args.exec(),
        }
    }
}
