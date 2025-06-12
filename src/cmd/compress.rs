use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

/// Compress the envelope or its subject.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Compress only the envelope's subject.
    #[arg(long, short, default_value = "false")]
    subject: bool,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        Ok(if self.subject {
            envelope.compress_subject()
        } else {
            envelope.compress()
        }?
        .ur_string())
    }
}
