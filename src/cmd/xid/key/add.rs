use bc_components::URI;
use bc_ur::prelude::*;
use bc_xid::{ Key, PrivateKeyOptions, XIDDocument };
use clap::Args;
use anyhow::Result ;

use crate::{
    cmd::xid::{
        key_args::{ KeyArgs, KeyArgsLike },
        key_privilege::KeyPrivilege,
        private_options::PrivateOptions,
        utils::{update_key, InputKey},
    },
    envelope_args::{ EnvelopeArgs, EnvelopeArgsLike },
};

/// Create a new XID document from an inception key, either ur:crypto-pubkeys or ur:crypto-prvkeys.
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

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let keys = self.read_key()?;

        let envelope = self.read_envelope()?;
        let mut xid_document = XIDDocument::from_unsigned_envelope(&envelope)?;

        let mut key = match &keys {
            InputKey::Private(private_key_base) => {
                Key::new_with_private_key(private_key_base.clone())
            }
            InputKey::Public(public_key_base) => {
                Key::new(public_key_base.clone())
            }
        };

        update_key(&mut key, self.name(), self.endpoints(), self.permissions());

        xid_document.add_key(key)?;

        let options = PrivateKeyOptions::from(self.private_opts());
        let unsigned_envelope = xid_document.to_unsigned_envelope_opt(options);
        let ur = UR::new("xid", unsigned_envelope.to_cbor())?;
        Ok(ur.string())
    }
}
