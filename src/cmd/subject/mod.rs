pub mod r#type;
pub mod assertion;

use clap::{Subcommand, Args};
use anyhow::Result;

/// Create an envelope with the given subject.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubjectCommands,
}

#[derive(Debug, Subcommand)]
enum SubjectCommands {
    Type(r#type::CommandArgs),
    Assertion(assertion::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubjectCommands::Type(args) => args.exec(),
            SubjectCommands::Assertion(args) => args.exec(),
        }
    }
}
