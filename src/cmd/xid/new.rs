use anyhow::Result;
use bc_components::URI;
use bc_xid::{GenesisMarkOptions, InceptionKeyOptions, XIDDocument};
use clap::Args;

use super::{
    key_args::{KeyArgs, KeyArgsLike},
    password_args::WritePasswordArgs,
    private_options::PrivateOptions,
    generator_options::GeneratorOptions,
    utils::{InputKey, update_key, xid_document_to_ur_string_with_password},
    xid_privilege::XIDPrivilege,
};

/// Create a new XID document from an inception key
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    key_args: KeyArgs,

    /// Whether to include, omit, or elide the provenance mark generator.
    #[arg(long = "generator", default_value = "include")]
    generator_opts: GeneratorOptions,

    #[command(flatten)]
    password_args: WritePasswordArgs,
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

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let keys = self.read_key()?;

        let mut xid_document = match &keys {
            InputKey::Private(private_key_base) => XIDDocument::new(
                InceptionKeyOptions::PrivateKeyBase(private_key_base.clone()),
                GenesisMarkOptions::None,
            ),
            InputKey::Public(public_keys) => XIDDocument::new(
                InceptionKeyOptions::PublicKeys(public_keys.clone()),
                GenesisMarkOptions::None,
            ),
        };

        let mut key = xid_document.keys().iter().next().unwrap().clone();
        xid_document.take_key(&key);
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
            &self.password_args,
        )
    }
}
