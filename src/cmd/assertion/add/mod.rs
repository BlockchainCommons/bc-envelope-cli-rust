pub mod predicate_object;
pub mod envelope;

use clap::{Subcommand, Args};

#[derive(Debug, Args)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<AddCommands>,
}

#[derive(Debug, Subcommand)]
enum AddCommands {
    Envelope(envelope::CommandArgs),
    PredicateObject(predicate_object::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
