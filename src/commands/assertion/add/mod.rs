pub mod predicate_object;
pub mod envelope;

use clap::{Subcommand, Args};

use self::{envelope::EnvelopeArgs, predicate_object::PredicateObjectArgs};

#[derive(Debug, Args)]
pub struct AddArgs {
    #[command(subcommand)]
    command: Option<AddCommands>,
}

#[derive(Debug, Subcommand)]
enum AddCommands {
    Envelope(EnvelopeArgs),
    PredicateObject(PredicateObjectArgs),
}

pub fn add_command(args: &AddArgs) {
    todo!();
}
