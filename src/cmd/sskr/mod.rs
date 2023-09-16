pub mod join;
pub mod split;

use clap::{Subcommand, Args};

/// Sharded Secret Key Reconstruction (SSKR).
#[derive(Debug, Args)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SskrCommands,
}

#[derive(Debug, Subcommand)]
enum SskrCommands {
    Split(split::CommandArgs),
    Join(join::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        match &self.command {
            SskrCommands::Split(args) => args.exec(),
            SskrCommands::Join(args) => args.exec(),
        }
    }
}
