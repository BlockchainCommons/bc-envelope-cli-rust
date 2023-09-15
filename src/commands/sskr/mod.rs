use clap::{Subcommand, Args};

/// Sharded Secret Key Reconstruction (SSKR).
#[derive(Debug, Args)]
pub struct SskrArgs {
    #[command(subcommand)]
    command: Option<SskrCommands>,
}

#[derive(Debug, Subcommand)]
enum SskrCommands {
    Split,
    Join,
}
pub fn sskr_command(args: &SskrArgs) {
    todo!();
}
