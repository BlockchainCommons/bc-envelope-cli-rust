use anyhow::Result;
use bc_components::URI;
use bc_envelope::known_values;
use bc_xid::{XIDDocument, XIDVerifySignature};
use clap::Args;

use crate::{EnvelopeArgs, EnvelopeArgsLike};

/// Retrieve all the XID document's resolution methods.
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
        let method_assertions =
            envelope.assertions_with_predicate(known_values::DEREFERENCE_VIA);
        let methods: Result<Vec<String>> = method_assertions
            .iter()
            .map(|method| {
                let uri = URI::try_from(method.try_object()?.try_leaf()?)?;
                Ok(uri.to_string())
            })
            .collect();
        let methods = methods?.join("\n");
        Ok(methods)
    }
}
