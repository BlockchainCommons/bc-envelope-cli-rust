use clap::{Subcommand, Args};

/// Sharded Secret Key Reconstruction (SSKR).
#[derive(Debug, Args)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<SskrCommands>,
}

#[derive(Debug, Subcommand)]
enum SskrCommands {
    Split,
    Join,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
