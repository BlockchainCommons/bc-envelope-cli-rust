use clap::{Subcommand, Args};

/// Utilities to generate and convert various objects.
#[derive(Debug, Args)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<GenerateCommands>,
}

#[derive(Debug, Subcommand)]
enum GenerateCommands {
    Arid,
    Digest,
    Key,
    Nonce,
    PrvKeys,
    PubKeys,
    Seed,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
