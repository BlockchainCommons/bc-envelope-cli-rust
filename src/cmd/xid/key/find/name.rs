use anyhow::Result;
use bc_envelope::EnvelopeEncodable;
use bc_ur::prelude::*;
use bc_xid::HasNickname;
use clap::Args;

use crate::{
    EnvelopeArgs, EnvelopeArgsLike,
    xid::{
        ReadPasswordArgs, VerifyArgs, XIDDocumentReadable, get_private_key_ur,
    },
};

/// Find the XID document's keys by assigned name.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    name: String,

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
    verify_args: VerifyArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let xid_document = self.read_xid_document_with_verify(
            self.verify_args.verify_signature(),
        )?;

        let keys = xid_document.keys();
        if self.private {
            // Return private keys
            let result = keys
                .iter()
                .filter(|key| key.nickname() == self.name)
                .map(|key| get_private_key_ur(key, &self.password_args))
                .collect::<Result<Vec<String>>>()?
                .join("\n");
            Ok(result)
        } else {
            // Return public keys (original behavior)
            let result = keys
                .iter()
                .filter_map(|key| {
                    if key.nickname() == self.name {
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
