pub mod cmd;
pub mod exec;

// use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use crate::exec::Exec;

/// A tool for manipulating the Envelope data type.
#[derive(Debug, Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<MainCommands>,

    #[command(flatten)]
    pub format: cmd::format::CommandArgs,
}

#[derive(Debug, Subcommand)]
enum MainCommands {
    Assertion(cmd::assertion::CommandArgs),
    Compress(cmd::compress::CommandArgs),
    Digest(cmd::digest::CommandArgs),
    Decrypt(cmd::decrypt::CommandArgs),
    Elide(cmd::elide::CommandArgs),
    Encrypt(cmd::encrypt::CommandArgs),
    Extract(cmd::extract::CommandArgs),
    Format(cmd::format::CommandArgs),
    Generate(cmd::generate::CommandArgs),
    Salt(cmd::salt::CommandArgs),
    Sign(cmd::sign::CommandArgs),
    Sskr(cmd::sskr::CommandArgs),
    Subject(cmd::subject::CommandArgs),
    Uncompress(cmd::uncompress::CommandArgs),
    Verify(cmd::verify::CommandArgs),
    // Diff,
    // Proof,
}

fn main() {
    let cli = Cli::parse();

    let command = cli.command.unwrap_or(MainCommands::Format(cli.format));
    println!("{command:?}");
    match command {
        MainCommands::Assertion(args) => args.exec(),
        MainCommands::Compress(args) => args.exec(),
        MainCommands::Digest(args) => args.exec(),
        MainCommands::Decrypt(args) => args.exec(),
        MainCommands::Elide(args) => args.exec(),
        MainCommands::Encrypt(args) => args.exec(),
        MainCommands::Extract(args) => args.exec(),
        MainCommands::Format(args) => args.exec(),
        MainCommands::Generate(args) => args.exec(),
        MainCommands::Salt(args) => args.exec(),
        MainCommands::Sign(args) => args.exec(),
        MainCommands::Sskr(args) => args.exec(),
        MainCommands::Subject(args) => args.exec(),
        MainCommands::Uncompress(args) => args.exec(),
        MainCommands::Verify(args) => args.exec(),
    }
}
