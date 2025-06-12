use std::collections::HashSet;

use anyhow::Result;
use bc_envelope::prelude::*;
use clap::{Args, ValueEnum};

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Depth {
    /// Return just the envelope's top digest.
    Top,
    /// Return the digests necessary to reveal the subject.
    Shallow,
    /// Return the digests needed to reveal the entire contents of the envelope.
    Deep,
}

/// Print the envelope's digest.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[arg(long = "depth", default_value = "top")]
    depth: Depth,

    #[arg(long = "hex", default_value = "false")]
    hex: bool,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        let digests: HashSet<Digest> = match self.depth {
            Depth::Top => vec![envelope.digest().into_owned()]
                .into_iter()
                .collect::<HashSet<_>>(),
            Depth::Shallow => envelope.shallow_digests(),
            Depth::Deep => envelope.deep_digests(),
        };
        let mut ordered_digests = digests.iter().cloned().collect::<Vec<_>>();
        ordered_digests.sort();
        let output = ordered_digests
            .iter()
            .map(|d| if self.hex { d.hex() } else { d.ur_string() })
            .collect::<Vec<String>>()
            .join(" ");
        Ok(output)
    }
}
