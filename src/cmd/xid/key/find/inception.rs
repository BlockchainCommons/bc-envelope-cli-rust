use anyhow::Result;
use bc_envelope::EnvelopeEncodable;
use bc_ur::prelude::*;
use clap::Args;

use crate::{
    cmd::xid::{
        password_args::ReadPasswordArgs,
        utils::{XIDDocumentReadable, get_private_key_ur},
    },
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

/// Find the XID document's inception key, if it exists.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Return the private key instead of the public key.
    ///
    /// For unencrypted keys, returns the PrivateKeys UR.
    /// For encrypted keys without --password, returns the encrypted envelope UR.
    /// For encrypted keys with --password, returns the decrypted PrivateKeys UR.
    #[arg(long)]
    private: bool,

    #[command(flatten)]
    password_args: ReadPasswordArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl XIDDocumentReadable for CommandArgs {}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let xid_document = self.read_xid_document()?;
        let result = if let Some(inception_key) = xid_document.inception_key() {
            if self.private {
                get_private_key_ur(inception_key, &self.password_args)?
            } else {
                inception_key.to_envelope().ur_string()
            }
        } else {
            "".to_string()
        };
        Ok(result)
    }
}
