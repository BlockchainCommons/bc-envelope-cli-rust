pub mod join;
pub mod split;
pub use anyhow::Result;
use clap::{Args, Subcommand};

/// Sharded Secret Key Reconstruction (SSKR).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SskrCommands,
}

#[derive(Debug, Subcommand)]
enum SskrCommands {
    Split(split::CommandArgs),
    Join(join::CommandArgs),
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            SskrCommands::Split(args) => args.exec(),
            SskrCommands::Join(args) => args.exec(),
        }
    }
}
