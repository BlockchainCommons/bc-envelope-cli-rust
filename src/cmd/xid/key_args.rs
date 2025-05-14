use bc_components::URI;
use bc_envelope::PublicKeys;
use clap::Args;
use anyhow::Result;

use super::{
    xid_privilege::XIDPrivilege,
    private_options::PrivateOptions,
    utils::{ read_key, read_public_key, InputKey },
};

pub trait KeyArgsLike {
    fn nickname(&self) -> &str;
    fn private_opts(&self) -> PrivateOptions;
    fn endpoints(&self) -> &[URI];
    fn permissions(&self) -> &[XIDPrivilege];
    fn keys(&self) -> Option<&str>;

    fn read_key(&self) -> Result<InputKey> {
        read_key(self.keys())
    }

    fn read_public_key(&self) -> Result<PublicKeys> {
        read_public_key(self.keys())
    }
}

#[derive(Debug, Args)]
#[group(skip)]
pub struct KeyArgs {
    /// A user-assigned name for the key.
    #[arg(long, default_value = "")]
    nickname: String,

    /// Whether to include, omit, or elide private keys.
    #[arg(long = "private", default_value = "include")]
    private_opts: PrivateOptions,

    /// Provide an endpoint for the key. May be repeated.
    #[arg(long = "endpoint", name = "URI", num_args = 1)]
    endpoints: Vec<URI>,

    /// Grant a specific permission to the key. May be repeated.
    #[arg(long = "allow", name = "PRIVILEGE", default_value = "all", num_args = 1)]
    permissions: Vec<XIDPrivilege>,

    /// The key to process. If omitted, the key will be read from stdin.
    #[arg(name = "KEYS")]
    keys: Option<String>,
}

impl KeyArgsLike for KeyArgs {
    fn nickname(&self) -> &str {
        &self.nickname
    }

    fn private_opts(&self) -> PrivateOptions {
        self.private_opts
    }

    fn endpoints(&self) -> &[URI] {
        &self.endpoints
    }

    fn permissions(&self) -> &[XIDPrivilege] {
        &self.permissions
    }

    fn keys(&self) -> Option<&str> {
        self.keys.as_deref()
    }
}
