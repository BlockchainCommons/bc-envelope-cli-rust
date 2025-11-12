use anyhow::Result;
use bc_envelope::EnvelopeEncodable;
use bc_ur::prelude::*;
use clap::Args;

use crate::{
    cmd::xid::{
        ReadPasswordArgs, XIDDocumentReadable, get_private_key_ur,
        read_public_key,
    },
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

/// Find the XID document's keys by their public key.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The key to process. If omitted, the key will be read from stdin.
    #[arg(name = "KEYS")]
    keys: Option<String>,

    /// Return the private key instead of the public key.
    ///
    /// For unencrypted keys, returns the PrivateKeys UR.
    /// For encrypted keys without --password, returns the encrypted envelope
    /// UR. For encrypted keys with --password, returns the decrypted
    /// PrivateKeys UR.
    #[arg(long)]
    private: bool,

    #[command(flatten)]
    password_args: ReadPasswordArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let public_keys = read_public_key(self.keys.as_deref())?;
        let xid_document = self.read_xid_document()?;

        let keys = xid_document.keys();
        if self.private {
            // Return private keys
            let result = keys
                .iter()
                .filter(|key| key.public_keys() == &public_keys)
                .map(|key| get_private_key_ur(key, &self.password_args))
                .collect::<Result<Vec<String>>>()?
                .join("\n");
            Ok(result)
        } else {
            // Return public keys (original behavior)
            let result = keys
                .iter()
                .filter_map(|key| {
                    if key.public_keys() == &public_keys {
                        Some(key.to_envelope().ur_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join("\n");
            Ok(result)
        }
    }
}
