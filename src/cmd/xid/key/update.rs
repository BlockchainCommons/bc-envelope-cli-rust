use bc_components::URI;
use bc_ur::prelude::*;
use bc_xid::PrivateKeyOptions;
use clap::Args;
use anyhow::{ Result, anyhow };

use crate::{
    cmd::xid::{
        key_args::{ KeyArgs, KeyArgsLike },
        key_privilege::KeyPrivilege,
        private_options::PrivateOptions,
        utils::{ update_key, XIDDocumentReadable},
    },
    envelope_args::{ EnvelopeArgs, EnvelopeArgsLike },
};

/// Updates the permissions, endpoints, or name of a key in a XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    key_args: KeyArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl KeyArgsLike for CommandArgs {
    fn name(&self) -> &str {
        self.key_args.name()
    }

    fn private_opts(&self) -> PrivateOptions {
        self.key_args.private_opts()
    }

    fn endpoints(&self) -> &[URI] {
        self.key_args.endpoints()
    }

    fn permissions(&self) -> &[KeyPrivilege] {
        self.key_args.permissions()
    }

    fn keys(&self) -> Option<&str> {
        self.key_args.keys()
    }
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl XIDDocumentReadable for CommandArgs { }

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let public_key_base = self.read_public_key()?;

        let mut xid_document = self.read_xid_document()?;

        let mut key = xid_document
            .find_key_by_public_key_base(&public_key_base)
            .cloned()
            .ok_or_else(|| anyhow!("Key not found in XID document"))?;

        xid_document.remove_key(&key);
        update_key(&mut key, self.name(), self.endpoints(), self.permissions());
        xid_document.add_key(key)?;

        let options = PrivateKeyOptions::from(self.private_opts());
        let unsigned_envelope = xid_document.to_unsigned_envelope_opt(options);
        let ur = UR::new("xid", unsigned_envelope.to_cbor())?;
        Ok(ur.string())
    }
}
