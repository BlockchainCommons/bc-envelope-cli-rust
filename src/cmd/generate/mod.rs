pub mod arid;
pub mod digest;
pub mod key;
pub mod nonce;
pub mod prv_keys;
pub mod pub_keys;
pub mod seed;

use clap::{Subcommand, Args};

/// Utilities to generate and convert various objects.
#[derive(Debug, Args)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Option<GenerateCommands>,
}

#[derive(Debug, Subcommand)]
enum GenerateCommands {
    Arid(arid::CommandArgs),
    Digest(digest::CommandArgs),
    Key(key::CommandArgs),
    Nonce(nonce::CommandArgs),
    PrvKeys(prv_keys::CommandArgs),
    PubKeys(pub_keys::CommandArgs),
    Seed(seed::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
