use bc_envelope::known_values;
use bc_ur::prelude::*;
use bc_xid::XIDDocument;
use clap::Args;
use anyhow::{Result, anyhow};

use crate::{cmd::xid::utils::XIDDocumentReadable, envelope_args::{ EnvelopeArgs, EnvelopeArgsLike }};

/// Retrieve the XID document's delegate at the specified index.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The index of the delegate to retrieve
    index: usize,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl XIDDocumentReadable for CommandArgs { }

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        XIDDocument::from_unsigned_envelope(&envelope)?; // Validation only
        let delegate_assertions = envelope.assertions_with_predicate(known_values::DELEGATE);
        let delegate_assertion = delegate_assertions.get(self.index).ok_or_else(|| anyhow!("Index out of bounds"))?;
        let delegate = delegate_assertion.try_object()?;
        Ok(delegate.ur_string())
    }
}
