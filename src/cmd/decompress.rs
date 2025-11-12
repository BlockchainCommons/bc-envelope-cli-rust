use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike};

/// Decompress the envelope or its subject.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Decompress only the envelope's subject.
    #[arg(long, short, default_value = "false")]
    subject: bool,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        Ok(if self.subject {
            envelope.decompress_subject()
        } else {
            envelope.decompress()
        }?
        .ur_string())
    }
}
