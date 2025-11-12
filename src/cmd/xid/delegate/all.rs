use anyhow::Result;
use bc_envelope::known_values;
use bc_ur::prelude::*;
use bc_xid::{XIDDocument, XIDVerifySignature};
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike, xid::XIDDocumentReadable};

/// Retrieve all delegates from the XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        XIDDocument::from_envelope(&envelope, None, XIDVerifySignature::None)?; // Validation only
        let delegate_assertions =
            envelope.assertions_with_predicate(known_values::DELEGATE);
        let delegates = delegate_assertions
            .iter()
            .map(|delegate| delegate.try_object().unwrap().ur_string())
            .collect::<Vec<_>>()
            .join("\n");
        Ok(delegates)
    }
}
