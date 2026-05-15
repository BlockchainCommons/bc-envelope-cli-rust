use anyhow::Result;
use bc_components::URI;
use bc_envelope::known_values;
use clap::Args;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike,
    xid::{xid_document_envelope, xid_from_document_envelope},
};

/// Retrieve all the XID resolution methods (dereferenceVia).
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
        xid_from_document_envelope(&envelope)?;
        let envelope = xid_document_envelope(&envelope)?;
        let resolution_assertions =
            envelope.assertions_with_predicate(known_values::DEREFERENCE_VIA);
        let methods = resolution_assertions
            .iter()
            .map(|assertion| {
                let uri: URI =
                    assertion.try_object()?.try_leaf()?.try_into()?;
                Ok(uri.to_string())
            })
            .collect::<Result<Vec<String>>>()?
            .join("\n");
        Ok(methods)
    }
}
