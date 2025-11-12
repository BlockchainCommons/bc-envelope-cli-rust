pub mod envelope;
pub mod pred_obj;

use anyhow::Result;
use clap::{Args, Subcommand};

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
    PredObj(pred_obj::CommandArgs),
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            AddCommands::Envelope(args) => args.exec(),
            AddCommands::PredObj(args) => args.exec(),
        }
    }
}
