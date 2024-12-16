use bc_envelope::known_values;
use bc_xid::XIDDocument;
use clap::Args;
use anyhow::Result;
use bc_ur::prelude::*;

use crate::{
    cmd::xid::utils::XIDDocumentReadable,
    envelope_args::{ EnvelopeArgs, EnvelopeArgsLike },
};

/// Retrieve all delegates from the XID document.
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

impl XIDDocumentReadable for CommandArgs {}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        XIDDocument::from_unsigned_envelope(&envelope)?; // Validation only
        let delegate_assertions = envelope.assertions_with_predicate(known_values::DELEGATE);
        let delegates = delegate_assertions
            .iter()
            .map(|delegate| delegate.try_object().unwrap().ur_string())
            .collect::<Vec<_>>()
            .join("\n");
        Ok(delegates)
    }
}
