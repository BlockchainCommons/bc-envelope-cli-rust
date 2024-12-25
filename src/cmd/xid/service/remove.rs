use bc_components::URI;
use clap::Args;
use anyhow::Result;

use crate::{cmd::xid::{private_options::PrivateOptions, utils::{read_uri, xid_document_to_ur_string, XIDDocumentReadable}}, envelope_args::{ EnvelopeArgs, EnvelopeArgsLike }};

/// Remove the given service from the XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The URI of the service to remove. If omitted, the URI will be will read from stdin.
    uri: Option<URI>,

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
        let uri = read_uri(self.uri.as_ref())?;
        let mut xid_document = self.read_xid_document()?;
        xid_document.remove_service(&uri)?;

        Ok(xid_document_to_ur_string(&xid_document, PrivateOptions::Include))
    }
}
