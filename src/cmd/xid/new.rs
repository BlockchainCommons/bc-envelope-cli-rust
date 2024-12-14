use bc_components::URI;
use bc_ur::prelude::*;
use bc_xid::{ PrivateKeyOptions, XIDDocument };
use clap::Args;
use anyhow::Result;

use super::{
    key_args::{ KeyArgs, KeyArgsLike },
    key_privilege::KeyPrivilege,
    private_options::PrivateOptions,
    utils::{update_key, InputKey},
};

/// Create a new XID document from an inception key
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    key_args: KeyArgs,
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

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let keys = self.read_key()?;

        let mut xid_document = match &keys {
            InputKey::Private(private_key_base) => {
                XIDDocument::new_with_private_key(private_key_base.clone())
            }
            InputKey::Public(public_key_base) => {
                XIDDocument::new(public_key_base.clone())
            }
        };

        let mut key = xid_document.keys().iter().next().unwrap().clone();
        xid_document.remove_key(&key);
        update_key(&mut key, self.name(), self.endpoints(), self.permissions());
        xid_document.add_key(key)?;

        let options = PrivateKeyOptions::from(self.private_opts());
        let unsigned_envelope = xid_document.to_unsigned_envelope_opt(options);
        let ur = UR::new("xid", unsigned_envelope.to_cbor())?;
        Ok(ur.string())
    }
}
