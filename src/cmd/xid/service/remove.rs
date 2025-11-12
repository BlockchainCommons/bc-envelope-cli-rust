use anyhow::Result;
use bc_components::URI;
use clap::Args;

use crate::{
    cmd::xid::{
        PrivateOptions, SigningArgs, VerifyArgs, XIDDocumentReadable, read_uri,
        xid_document_to_ur_string,
    },
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

/// Remove the given service from the XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The URI of the service to remove. If omitted, the URI will be will read
    /// from stdin.
    uri: Option<URI>,

    #[command(flatten)]
    verify_args: VerifyArgs,

    #[command(flatten)]
    signing_args: SigningArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let uri = read_uri(self.uri.as_ref())?;
        let mut xid_document = self.read_xid_document_with_verify(
            self.verify_args.verify_signature(),
        )?;
        xid_document.remove_service(&uri)?;

        let signing_options = self.signing_args.signing_options(None)?;

        xid_document_to_ur_string(
            &xid_document,
            PrivateOptions::Include,
            None,
            None,
            None,
            signing_options,
        )
    }
}
