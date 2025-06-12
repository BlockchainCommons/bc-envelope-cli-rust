use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

/// Retrieve all the envelope's assertions.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        let assertions = envelope.assertions();
        let output = assertions
            .iter()
            .map(|a| a.ur_string())
            .collect::<Vec<String>>()
            .join("\n");
        Ok(output)
    }
}
