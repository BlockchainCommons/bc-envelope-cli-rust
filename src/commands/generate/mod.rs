use clap::{Subcommand, Args};

/// Utilities to generate and convert various objects.
#[derive(Debug, Args)]
pub struct GenerateArgs {
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

pub fn generate_command(args: &GenerateArgs) {
    todo!();
}
