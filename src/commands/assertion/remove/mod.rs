pub mod envelope;
pub mod predicate_object;

use clap::{Subcommand, Args};

use self::{envelope::EnvelopeArgs, predicate_object::PredicateObjectArgs};

#[derive(Debug, Args)]
pub struct RemoveArgs {
    #[command(subcommand)]
    command: Option<RemoveCommands>,
}

#[derive(Debug, Subcommand)]
enum RemoveCommands {
    Envelope(EnvelopeArgs),
    PredicateObject(PredicateObjectArgs),
}

pub fn remove_command(args: &RemoveArgs) {
    todo!();
}
