use bc_envelope::known_values;
use bc_ur::prelude::*;
use bc_xid::XIDDocument;
use clap::Args;
use anyhow::Result;

use crate::envelope_args::{ EnvelopeArgs, EnvelopeArgsLike };

/// Retrieve all the XID services.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        XIDDocument::from_unsigned_envelope(&envelope)?; // Validation only
        let service_assertions = envelope.assertions_with_predicate(known_values::SERVICE);
        let services = service_assertions
            .iter()
            .map(|key| key.try_object().unwrap().ur_string())
            .collect::<Vec<String>>()
            .join("\n");
        Ok(services)
    }
}
