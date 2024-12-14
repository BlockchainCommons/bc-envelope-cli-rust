use bc_ur::prelude::*;
use bc_xid::PrivateKeyOptions;
use clap::Args;
use anyhow::{Result, anyhow};

use crate::{cmd::xid::utils::{read_public_key, XIDDocumentReadable}, envelope_args::{ EnvelopeArgs, EnvelopeArgsLike }};

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
        let public_key_base = read_public_key(self.keys.as_deref())?;
        let mut xid_document = self.read_xid_document()?;

        let key = xid_document
            .find_key_by_public_key_base(&public_key_base)
            .cloned()
            .ok_or_else(|| anyhow!("Key not found in XID document"))?;

        xid_document.remove_key(&key);
        let unsigned_envelope = xid_document.to_unsigned_envelope_opt(PrivateKeyOptions::Include);
        let ur = UR::new("xid", unsigned_envelope.to_cbor())?;
        Ok(ur.string())
    }
}
