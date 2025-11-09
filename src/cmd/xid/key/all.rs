use anyhow::Result;
use bc_envelope::known_values;
use bc_ur::prelude::*;
use bc_xid::{XIDDocument, XIDVerifySignature};
use clap::Args;

use crate::{
    cmd::xid::{
        password_args::ReadPasswordArgs,
        utils::{XIDDocumentReadable, get_private_key_ur},
    },
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

/// Retrieve all the XID document's keys.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Return private keys instead of public keys.
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
        if self.private {
            // Return private keys
            let xid_document = self.read_xid_document()?;
            let keys = xid_document
                .keys()
                .iter()
                .map(|key| get_private_key_ur(key, &self.password_args))
                .collect::<Result<Vec<String>>>()?
                .join("\n");
            Ok(keys)
        } else {
            // Return public keys (original behavior)
            let envelope = self.read_envelope()?;
            XIDDocument::from_envelope(
                &envelope,
                None,
                XIDVerifySignature::None,
            )?; // Validation only
            let key_assertions =
                envelope.assertions_with_predicate(known_values::KEY);
            let keys = key_assertions
                .iter()
                .map(|key| key.try_object().unwrap().ur_string())
                .collect::<Vec<String>>()
                .join("\n");
            Ok(keys)
        }
    }
}
