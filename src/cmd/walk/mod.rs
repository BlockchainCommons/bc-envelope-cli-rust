pub mod decompress;
pub mod decrypt;
pub mod matching;
pub mod replace;
pub mod unelide;

use std::collections::HashSet;

use anyhow::Result;
use bc_envelope::prelude::*;
use clap::{Args, Subcommand};

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

/// Walk an envelope's nodes.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Optional target digests to filter nodes (space-separated UR digests).
    #[arg(long, value_delimiter = ' ')]
    target: Vec<String>,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,

    #[command(subcommand)]
    command: Option<WalkCommands>,
}

#[derive(Debug, Subcommand)]
enum WalkCommands {
    Matching(matching::CommandArgs),
    Unelide(unelide::CommandArgs),
    Decrypt(decrypt::CommandArgs),
    Decompress(decompress::CommandArgs),
    Replace(replace::CommandArgs),
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        match &self.command {
            Some(WalkCommands::Matching(args)) => args
                .exec_with_envelope_and_target(
                    self.read_envelope()?,
                    parse_target_digests(&self.target)?,
                ),
            Some(WalkCommands::Unelide(args)) => {
                args.exec_with_envelope(self.read_envelope()?)
            }
            Some(WalkCommands::Decrypt(args)) => {
                args.exec_with_envelope(self.read_envelope()?)
            }
            Some(WalkCommands::Decompress(args)) => args
                .exec_with_envelope_and_target(
                    self.read_envelope()?,
                    parse_target_digests(&self.target)?,
                ),
            Some(WalkCommands::Replace(args)) => args
                .exec_with_envelope_and_target(
                    self.read_envelope()?,
                    parse_target_digests(&self.target)?,
                ),
            None => {
                // Default: output all digests
                let envelope = self.read_envelope()?;
                let target = parse_target_digests(&self.target)?;
                let digests = envelope.nodes_matching(target.as_ref(), &[]);
                output_digests(digests)
            }
        }
    }
}

fn parse_target_digests(target: &[String]) -> Result<Option<HashSet<Digest>>> {
    if target.is_empty() {
        return Ok(None);
    }

    let mut digests = HashSet::new();
    for ur_string in target {
        let digest = Digest::from_ur_string(ur_string)?;
        digests.insert(digest);
    }
    Ok(Some(digests))
}

fn output_digests(digests: HashSet<Digest>) -> Result<String> {
    let mut ordered_digests = digests.iter().cloned().collect::<Vec<_>>();
    ordered_digests.sort();
    let output = ordered_digests
        .iter()
        .map(|d| d.ur_string())
        .collect::<Vec<_>>()
        .join(" ");
    Ok(output)
}
