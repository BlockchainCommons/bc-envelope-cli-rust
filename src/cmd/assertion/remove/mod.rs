pub mod envelope;
pub mod pred_obj;

use anyhow::Result;
use clap::{Args, Subcommand};

/// Remove an assertion from the given envelope.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Envelope(envelope::CommandArgs),
    PredObj(pred_obj::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SubCommands::Envelope(args) => args.exec(),
            SubCommands::PredObj(args) => args.exec(),
        }
    }
}
