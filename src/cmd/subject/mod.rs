pub mod single;
pub mod assertion;

use clap::{Subcommand, Args};

/// Create an envelope with the given subject.
#[derive(Debug, Args)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<SubjectCommands>,
}

#[derive(Debug, Subcommand)]
enum SubjectCommands {
    Single(single::CommandArgs),
    Assertion(assertion::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
