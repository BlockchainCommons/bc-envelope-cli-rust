pub mod envelope;
pub mod pred_obj;

use clap::{Subcommand, Args};

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
    fn exec(&self) -> anyhow::Result<String> {
        match &self.command {
            SubCommands::Envelope(args) => args.exec(),
            SubCommands::PredObj(args) => args.exec(),
        }
    }
}
