use bc_envelope::known_values;
use bc_ur::prelude::*;
use bc_xid::XIDDocument;
use clap::Args;
use anyhow::{anyhow, Result};

use crate::envelope_args::{ EnvelopeArgs, EnvelopeArgsLike };

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
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        XIDDocument::from_unsigned_envelope(&envelope)?; // Validation only
        let key_assertions = envelope.assertions_with_predicate(known_values::KEY);
        let key_assertion = key_assertions.get(self.index).ok_or_else(|| anyhow!("Index out of bounds"))?;
        let key = key_assertion.try_object()?;
        Ok(key.ur_string())
    }
}
