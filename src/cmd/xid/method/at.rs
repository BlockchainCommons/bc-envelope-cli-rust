use anyhow::{Result, anyhow};
use bc_components::URI;
use bc_envelope::known_values;
use bc_xid::{XIDDocument, XIDVerifySignature};
use clap::Args;

use crate::{
    cmd::xid::XIDDocumentReadable,
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

/// Retrieve the resolution method at the given index
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The index of the resolution method to retrieve
    index: usize,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        XIDDocument::from_envelope(&envelope, None, XIDVerifySignature::None)?; // Validation only
        let method_assertions =
            envelope.assertions_with_predicate(known_values::DEREFERENCE_VIA);
        let method_assertion = method_assertions
            .get(self.index)
            .ok_or_else(|| anyhow!("Index out of bounds"))?;
        let uri: URI = method_assertion.try_object()?.try_leaf()?.try_into()?;
        Ok(uri.to_string())
    }
}
