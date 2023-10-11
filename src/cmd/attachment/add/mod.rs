pub mod components;
pub mod envelope;

use clap::{Subcommand, Args};

/// Add an assertion to the given envelope.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: AddCommands,
}

#[derive(Debug, Subcommand)]
enum AddCommands {
    Envelope(envelope::CommandArgs),
    Components(components::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        match &self.command {
            AddCommands::Envelope(args) => args.exec(),
            AddCommands::Components(args) => args.exec(),
        }
    }
}
