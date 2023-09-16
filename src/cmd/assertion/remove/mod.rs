pub mod envelope;
pub mod predicate_object;

use clap::{Subcommand, Args};

#[derive(Debug, Args)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Envelope(envelope::CommandArgs),
    PredicateObject(predicate_object::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) {
        todo!();
    }
}
