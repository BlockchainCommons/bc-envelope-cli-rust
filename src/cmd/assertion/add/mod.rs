pub mod predicate_object;
pub mod envelope;

use clap::{Subcommand, Args};

/// (DEFAULT) Add an assertion to the given envelope.
#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<SubCommands>,

    #[command(flatten)]
    pub default_command: predicate_object::CommandArgs,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Envelope(envelope::CommandArgs),
    PredicateObject(predicate_object::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        match self.command.as_ref() {
            Some(SubCommands::Envelope(args)) => args.exec(),
            Some(SubCommands::PredicateObject(args)) => args.exec(),
            None => self.default_command.exec(),
        }
    }
}
