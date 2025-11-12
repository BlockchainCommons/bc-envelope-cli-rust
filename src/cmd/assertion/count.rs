use anyhow::Result;
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike};

/// Print the count of the envelope's assertions.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        Ok(envelope.assertions().len().to_string())
    }
}
