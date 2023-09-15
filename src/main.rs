pub mod commands;

// use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use commands::{assertion::AssertionArgs, compress::CompressArgs, digest::DigestArgs, decrypt::DecryptArgs, elide::ElideArgs, encrypt::EncryptArgs, extract::ExtractArgs, generate::GenerateArgs, salt::SaltArgs, sign::SignArgs, sskr::SskrArgs, subject::SubjectArgs, uncompress::UncompressArgs, verify::VerifyArgs, format::FormatArgs};

use crate::commands::{format::format_command, assertion::assertion_command, compress::compress_command, digest::digest_command, decrypt::decrypt_command, elide::elide_command, encrypt::encrypt_command, extract::extract_command, generate::generate_command, salt::salt_command, sign::sign_command, sskr::sskr_command, subject::subject_command, uncompress::uncompress_command, verify::verify_command};

/// A tool for manipulating the Envelope data type.
#[derive(Debug, Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<MainCommands>,

    #[command(flatten)]
    pub format: FormatArgs,
}

#[derive(Debug, Subcommand)]
enum MainCommands {
    Assertion(AssertionArgs),
    Compress(CompressArgs),
    Digest(DigestArgs),
    Decrypt(DecryptArgs),
    Elide(ElideArgs),
    Encrypt(EncryptArgs),
    Extract(ExtractArgs),
    Format(FormatArgs),
    Generate(GenerateArgs),
    Salt(SaltArgs),
    Sign(SignArgs),
    Sskr(SskrArgs),
    Subject(SubjectArgs),
    Uncompress(UncompressArgs),
    Verify(VerifyArgs),
    // Diff,
    // Proof,
}

fn main() {
    let cli = Cli::parse();

    let command = cli.command.unwrap_or(MainCommands::Format(cli.format));
    println!("{command:?}");
    match command {
        MainCommands::Assertion(args) => assertion_command(&args),
        MainCommands::Compress(args) => compress_command(&args),
        MainCommands::Digest(args) => digest_command(&args),
        MainCommands::Decrypt(args) => decrypt_command(&args),
        MainCommands::Elide(args) => elide_command(&args),
        MainCommands::Encrypt(args) => encrypt_command(&args),
        MainCommands::Extract(args) => extract_command(&args),
        MainCommands::Format(args) => format_command(&args),
        MainCommands::Generate(args) => generate_command(&args),
        MainCommands::Salt(args) => salt_command(&args),
        MainCommands::Sign(args) => sign_command(&args),
        MainCommands::Sskr(args) => sskr_command(&args),
        MainCommands::Subject(args) => subject_command(&args),
        MainCommands::Uncompress(args) => uncompress_command(&args),
        MainCommands::Verify(args) => verify_command(&args),
    }
}
