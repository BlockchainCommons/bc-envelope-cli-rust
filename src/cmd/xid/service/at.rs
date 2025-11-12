use anyhow::{Result, anyhow};
use bc_envelope::known_values;
use bc_ur::prelude::*;
use bc_xid::{XIDDocument, XIDVerifySignature};
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike};

/// Retrieve the XID Document's service at the given index
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The index of the key to retrieve
    index: usize,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        XIDDocument::from_envelope(&envelope, None, XIDVerifySignature::None)?; // Validation only
        let service_assertions =
            envelope.assertions_with_predicate(known_values::SERVICE);
        let service_assertion = service_assertions
            .get(self.index)
            .ok_or_else(|| anyhow!("Index out of bounds"))?;
        let service = service_assertion.try_object()?;
        Ok(service.ur_string())
    }
}
