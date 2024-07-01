pub mod arid;
pub mod digest;
pub mod key;
pub mod nonce;
pub mod prv_keys;
pub mod pub_keys;
pub mod seed;
pub mod signer;
pub mod verifier;
pub mod signer_type;
pub use signer_type::SignerType;

use clap::{Subcommand, Args};
use anyhow::Result;

/// Utilities to generate and convert various objects.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: GenerateCommands,
}

#[derive(Debug, Subcommand)]
enum GenerateCommands {
    Arid(arid::CommandArgs),
    Digest(digest::CommandArgs),
    Key(key::CommandArgs),
    Nonce(nonce::CommandArgs),
    #[command(id = "prvkeys")]
    PrvKeys(prv_keys::CommandArgs),
    #[command(id = "pubkeys")]
    PubKeys(pub_keys::CommandArgs),
    Signer(signer::CommandArgs),
    Verifier(verifier::CommandArgs),
    Seed(seed::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            GenerateCommands::Arid(args) => args.exec(),
            GenerateCommands::Digest(args) => args.exec(),
            GenerateCommands::Key(args) => args.exec(),
            GenerateCommands::Nonce(args) => args.exec(),
            GenerateCommands::PrvKeys(args) => args.exec(),
            GenerateCommands::PubKeys(args) => args.exec(),
            GenerateCommands::Signer(args) => args.exec(),
            GenerateCommands::Verifier(args) => args.exec(),
            GenerateCommands::Seed(args) => args.exec(),
        }
    }
}
