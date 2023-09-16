pub mod predicate;
pub mod object;

use clap::{Subcommand, Args};

/// Find all assertions matching the given criteria.
#[derive(Debug, Args)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Object(object::CommandArgs),
    Predicate(predicate::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
