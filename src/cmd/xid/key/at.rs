use anyhow::{Result, anyhow};
use bc_envelope::known_values;
use bc_ur::prelude::*;
use bc_xid::XIDDocument;
use clap::Args;

use crate::{
    cmd::xid::{
        password_args::ReadPasswordArgs,
        utils::{XIDDocumentReadable, get_private_key_ur},
    },
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

/// Retrieve the XID Document's key at the given index
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The index of the key to retrieve
    index: usize,

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
        if self.private {
            // Return private key
            let xid_document = self.read_xid_document()?;
            let key = xid_document
                .keys()
                .iter()
                .nth(self.index)
                .ok_or_else(|| anyhow!("Index out of bounds"))?;
            get_private_key_ur(key, &self.password_args)
        } else {
            // Return public key (original behavior)
            let envelope = self.read_envelope()?;
            XIDDocument::from_unsigned_envelope(&envelope)?; // Validation only
            let key_assertions =
                envelope.assertions_with_predicate(known_values::KEY);
            let key_assertion = key_assertions
                .get(self.index)
                .ok_or_else(|| anyhow!("Index out of bounds"))?;
            let key = key_assertion.try_object()?;
            Ok(key.ur_string())
        }
    }
}
