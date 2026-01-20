use anyhow::Result;
use bc_components::URI;
use bc_envelope::known_values;
use bc_xid::{XIDDocument, XIDVerifySignature};
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike};

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
        XIDDocument::from_envelope(&envelope, None, XIDVerifySignature::None)?; // Validation only
        let resolution_assertions =
            envelope.assertions_with_predicate(known_values::DEREFERENCE_VIA);
        let methods = resolution_assertions
            .iter()
            .map(|assertion| {
                let uri: URI = assertion
                    .try_object()
                    .unwrap()
                    .try_leaf()
                    .unwrap()
                    .try_into()
                    .unwrap();
                uri.to_string()
            })
            .collect::<Vec<String>>()
            .join("\n");
        Ok(methods)
    }
}
