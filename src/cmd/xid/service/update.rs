use bc_components::URI;
use clap::Args;
use anyhow::{ Result, anyhow };

use crate::{
    cmd::xid::{
        key_args::{ KeyArgs, KeyArgsLike }, private_options::PrivateOptions, utils::{ update_key, xid_document_to_ur_string, XIDDocumentReadable}, xid_privilege::XIDPrivilege
    },
    envelope_args::{ EnvelopeArgs, EnvelopeArgsLike },
};

/// Updates the permissions, delegates, keys, capability identifer, or name of a service in a XID document.
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

    fn permissions(&self) -> &[XIDPrivilege] {
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
            .ok_or_else(|| anyhow!("Key not found"))?;

        xid_document.take_key(&key);
        update_key(&mut key, self.name(), self.endpoints(), self.permissions());
        xid_document.add_key(key)?;

        Ok(xid_document_to_ur_string(&xid_document, self.private_opts()))
    }
}
