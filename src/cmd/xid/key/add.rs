use anyhow::Result;
use bc_components::URI;
use bc_xid::Key;
use clap::Args;

use crate::{
    cmd::xid::{
        key_args::{KeyArgs, KeyArgsLike},
        password_args::ReadWritePasswordArgs,
        private_options::PrivateOptions,
        utils::{
            InputKey, XIDDocumentReadable, update_key,
            xid_document_to_ur_string_with_password,
        },
        xid_privilege::XIDPrivilege,
    },
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
};

/// Add a key to the XID document.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    key_args: KeyArgs,

    #[command(flatten)]
    password_args: ReadWritePasswordArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl KeyArgsLike for CommandArgs {
    fn nickname(&self) -> &str {
        self.key_args.nickname()
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

impl XIDDocumentReadable for CommandArgs {}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let keys = self.read_key()?;

        let mut xid_document =
            self.read_xid_document_with_password(&self.password_args.read)?;

        let mut key = match &keys {
            InputKey::Private(private_key_base) => {
                Key::new_with_private_key_base(private_key_base.clone())
            }
            InputKey::Public(public_keys) => Key::new(public_keys.clone()),
        };

        update_key(
            &mut key,
            self.nickname(),
            self.endpoints(),
            self.permissions(),
        );

        xid_document.add_key(key)?;

        xid_document_to_ur_string_with_password(
            &xid_document,
            self.private_opts(),
            &self.password_args.write,
        )
    }
}
