use clap::Args;
use anyhow::Result;

use crate::{cmd::xid::{private_options::PrivateOptions, utils::{read_public_key, xid_document_to_ur_string, XIDDocumentReadable}}, envelope_args::{ EnvelopeArgs, EnvelopeArgsLike }};

/// Remove the given key from the XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The key to remove. If omitted, the key will be will read from stdin.
    #[arg(name = "KEYS")]
    keys: Option<String>,

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
        let public_keys = read_public_key(self.keys.as_deref())?;
        let mut xid_document = self.read_xid_document()?;
        xid_document.remove_key(&public_keys)?;

        Ok(xid_document_to_ur_string(&xid_document, PrivateOptions::Include))
    }
}
