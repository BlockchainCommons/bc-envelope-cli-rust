use bc_envelope::EnvelopeEncodable;
use bc_ur::prelude::*;
use clap::Args;
use anyhow::Result;

use crate::{cmd::xid::utils::{read_public_key, XIDDocumentReadable}, envelope_args::{ EnvelopeArgs, EnvelopeArgsLike }};

/// Find the XID document's keys by their public key.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The key to process. If omitted, the key will be read from stdin.
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
        let public_key_base = read_public_key(self.keys.as_deref())?;
        let xid_document = self.read_xid_document()?;

        let keys = xid_document.keys();
        let result = keys.iter().filter_map(|key| {
            if key.public_key_base() == &public_key_base {
                Some(key.to_envelope().ur_string())
            } else {
                None
            }
        }).collect::<Vec<String>>().join("\n");
        Ok(result)
    }
}
