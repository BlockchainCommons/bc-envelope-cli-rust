use bc_components::URI;
use bc_xid::XIDDocument;
use clap::Args;
use anyhow::Result;

use super::{
    key_args::{ KeyArgs, KeyArgsLike }, private_options::PrivateOptions, utils::{update_key, xid_document_to_ur_string, InputKey}, xid_privilege::XIDPrivilege
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

    fn permissions(&self) -> &[XIDPrivilege] {
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
                XIDDocument::new_with_private_key_base(private_key_base.clone())
            }
            InputKey::Public(public_keys) => {
                XIDDocument::new(public_keys.clone())
            }
        };

        let mut key = xid_document.keys().iter().next().unwrap().clone();
        xid_document.take_key(&key);
        update_key(&mut key, self.name(), self.endpoints(), self.permissions());
        xid_document.add_key(key)?;

        Ok(xid_document_to_ur_string(&xid_document, self.private_opts()))
    }
}
