use bc_components::URI;
use clap::Args;
use anyhow::Result;

use super::{key_privilege::KeyPrivilege, private_options::PrivateOptions, utils::{read_key, InputKey}};

pub trait KeyArgsLike {
    fn name(&self) -> &str;
    fn private_opts(&self) -> PrivateOptions;
    fn endpoints(&self) -> &[URI];
    fn permissions(&self) -> &[KeyPrivilege];
    fn keys(&self) -> Option<&str>;

    fn read_key(&self) -> Result<InputKey> {
        read_key(self.keys())
    }
}


#[derive(Debug, Args)]
#[group(skip)]
pub struct KeyArgs {
    /// A user-assigned name for the key.
    #[arg(long, default_value = "")]
    name: String,

    /// Whether to include, omit, or elide private keys.
    #[arg(long = "private", default_value = "include")]
    private_opts: PrivateOptions,

    /// Provide one or more endpoints for the key.
    #[arg(long, name = "URI", num_args = 1..)]
    endpoints: Vec<URI>,

    /// Grant specific permissions to the key.
    #[arg(long, name = "PRIVILEGE", default_value = "all", num_args = 1..)]
    permissions: Vec<KeyPrivilege>,

    /// The public or private key base to convert, either ur:crypto-pubkeys or ur:crypto-prvkeys.
    #[arg(name = "KEYS")]
    keys: Option<String>,
}

impl KeyArgsLike for KeyArgs {
    fn name(&self) -> &str {
        &self.name
    }

    fn private_opts(&self) -> PrivateOptions {
        self.private_opts
    }

    fn endpoints(&self) -> &[URI] {
        &self.endpoints
    }

    fn permissions(&self) -> &[KeyPrivilege] {
        &self.permissions
    }

    fn keys(&self) -> Option<&str> {
        self.keys.as_deref()
    }
}
