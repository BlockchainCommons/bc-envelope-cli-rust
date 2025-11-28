//! A command line tool for manipulating the Gordian Envelope data type. See the main repo [README](https://github.com/BlockchainCommons/bc-envelope-cli-rust/blob/master/README.md).

#[doc(hidden)]
mod cmd;
pub use cmd::*;
#[doc(hidden)]
mod data_types;
pub use data_types::*;
#[doc(hidden)]
mod envelope_args;
pub use envelope_args::*;
#[doc(hidden)]
mod exec;
pub use exec::*;
#[doc(hidden)]
mod pred_obj_args;
pub use pred_obj_args::*;
#[doc(hidden)]
mod styles;
#[doc(hidden)]
mod subject_args;
pub use subject_args::*;
#[doc(hidden)]
mod utils;
use anyhow::Result;
use clap::{Parser, Subcommand};
pub use utils::*;

use crate::Exec;

/// A tool for manipulating the Gordian Envelope data type.
#[derive(Debug, Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(infer_subcommands = true)]
#[command(styles=styles::get_styles())]
#[doc(hidden)]
struct Cli {
    #[command(subcommand)]
    command: MainCommands,
}

#[derive(Debug, Subcommand)]
#[doc(hidden)]
enum MainCommands {
    Assertion(assertion::CommandArgs),
    Attachment(attachment::CommandArgs),
    Compress(compress::CommandArgs),
    Decrypt(decrypt::CommandArgs),
    Digest(digest::CommandArgs),
    Elide(elide::CommandArgs),
    Encrypt(encrypt::CommandArgs),
    Export(export::CommandArgs),
    Extract(extract::CommandArgs),
    Format(format::CommandArgs),
    Generate(generate::CommandArgs),
    Import(import::CommandArgs),
    Info(info::CommandArgs),
    Match(pattern::CommandArgs),
    Proof(proof::CommandArgs),
    Salt(salt::CommandArgs),
    Sign(sign::CommandArgs),
    Sskr(sskr::CommandArgs),
    Subject(subject::CommandArgs),
    Decompress(decompress::CommandArgs),
    Verify(verify::CommandArgs),
    Walk(walk::CommandArgs),
    Xid(xid::CommandArgs),
}

#[doc(hidden)]
fn main() -> Result<()> {
    bc_components::register_tags();
    bc_envelope::register_tags();
    provenance_mark::register_tags();

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
        MainCommands::Match(args) => args.exec(),
        MainCommands::Proof(args) => args.exec(),
        MainCommands::Salt(args) => args.exec(),
        MainCommands::Sign(args) => args.exec(),
        MainCommands::Sskr(args) => args.exec(),
        MainCommands::Subject(args) => args.exec(),
        MainCommands::Decompress(args) => args.exec(),
        MainCommands::Verify(args) => args.exec(),
        MainCommands::Walk(args) => args.exec(),
        MainCommands::Xid(args) => args.exec(),
    };
    let output = output?;
    if !output.is_empty() {
        println!("{}", output);
    }
    Ok(())
}
