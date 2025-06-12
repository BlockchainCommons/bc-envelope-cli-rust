//! A command line tool for manipulating the Gordian Envelope data type. See the main repo [README](https://github.com/BlockchainCommons/bc-envelope-cli-rust/blob/master/README.md).

#[doc(hidden)]
mod cmd;
#[doc(hidden)]
mod data_types;
#[doc(hidden)]
mod envelope_args;
#[doc(hidden)]
mod exec;
#[doc(hidden)]
mod pred_obj_args;
#[doc(hidden)]
mod styles;
#[doc(hidden)]
mod subject_args;
#[doc(hidden)]
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::exec::Exec;

/// A tool for manipulating the Gordian Envelope data type.
#[derive(Debug, Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(styles=styles::get_styles())]
#[doc(hidden)]
struct Cli {
    #[command(subcommand)]
    command: MainCommands,
}

#[derive(Debug, Subcommand)]
#[doc(hidden)]
enum MainCommands {
    Assertion(cmd::assertion::CommandArgs),
    Attachment(cmd::attachment::CommandArgs),
    Compress(cmd::compress::CommandArgs),
    Decrypt(cmd::decrypt::CommandArgs),
    Digest(cmd::digest::CommandArgs),
    Elide(cmd::elide::CommandArgs),
    Encrypt(cmd::encrypt::CommandArgs),
    Export(cmd::export::CommandArgs),
    Extract(cmd::extract::CommandArgs),
    Format(cmd::format::CommandArgs),
    Generate(cmd::generate::CommandArgs),
    Import(cmd::import::CommandArgs),
    Info(cmd::info::CommandArgs),
    Proof(cmd::proof::CommandArgs),
    Salt(cmd::salt::CommandArgs),
    Sign(cmd::sign::CommandArgs),
    Sskr(cmd::sskr::CommandArgs),
    Subject(cmd::subject::CommandArgs),
    Uncompress(cmd::uncompress::CommandArgs),
    Verify(cmd::verify::CommandArgs),
    Xid(cmd::xid::CommandArgs),
}

#[doc(hidden)]
fn main() -> Result<()> {
    bc_envelope::register_tags();

    let cli = Cli::parse();

    let output = match cli.command {
        MainCommands::Assertion(args) => args.exec(),
        MainCommands::Attachment(args) => args.exec(),
        MainCommands::Compress(args) => args.exec(),
        MainCommands::Decrypt(args) => args.exec(),
        MainCommands::Digest(args) => args.exec(),
        MainCommands::Elide(args) => args.exec(),
        MainCommands::Encrypt(args) => args.exec(),
        MainCommands::Export(args) => args.exec(),
        MainCommands::Extract(args) => args.exec(),
        MainCommands::Format(args) => args.exec(),
        MainCommands::Generate(args) => args.exec(),
        MainCommands::Import(args) => args.exec(),
        MainCommands::Info(args) => args.exec(),
        MainCommands::Proof(args) => args.exec(),
        MainCommands::Salt(args) => args.exec(),
        MainCommands::Sign(args) => args.exec(),
        MainCommands::Sskr(args) => args.exec(),
        MainCommands::Subject(args) => args.exec(),
        MainCommands::Uncompress(args) => args.exec(),
        MainCommands::Verify(args) => args.exec(),
        MainCommands::Xid(args) => args.exec(),
    };
    let output = output?;
    if !output.is_empty() {
        println!("{}", output);
    }
    Ok(())
}
