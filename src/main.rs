pub mod cmd;
pub mod exec;
pub mod styles;

use std::error::Error;

// use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use crate::exec::Exec;

/// A tool for manipulating the Envelope data type.
#[derive(Debug, Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(args_conflicts_with_subcommands = true)]
#[command(styles=styles::get_styles())]
struct Cli {
    #[command(subcommand)]
    command: Option<MainCommands>,

    #[command(flatten)]
    pub default_command: cmd::format::CommandArgs,
}

#[derive(Debug, Subcommand)]
enum MainCommands {
    Assertion(cmd::assertion::CommandArgs),
    Compress(cmd::compress::CommandArgs),
    Decrypt(cmd::decrypt::CommandArgs),
    Digest(cmd::digest::CommandArgs),
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
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let command = cli.command.unwrap_or(MainCommands::Format(cli.default_command));
    let output = match command {
        MainCommands::Assertion(args) => args.exec()?,
        MainCommands::Compress(args) => args.exec()?,
        MainCommands::Decrypt(args) => args.exec()?,
        MainCommands::Digest(args) => args.exec()?,
        MainCommands::Elide(args) => args.exec()?,
        MainCommands::Encrypt(args) => args.exec()?,
        MainCommands::Extract(args) => args.exec()?,
        MainCommands::Format(args) => args.exec()?,
        MainCommands::Generate(args) => args.exec()?,
        MainCommands::Salt(args) => args.exec()?,
        MainCommands::Sign(args) => args.exec()?,
        MainCommands::Sskr(args) => args.exec()?,
        MainCommands::Subject(args) => args.exec()?,
        MainCommands::Uncompress(args) => args.exec()?,
        MainCommands::Verify(args) => args.exec()?,
    };
    println!("{}", output);
    Ok(())
}
