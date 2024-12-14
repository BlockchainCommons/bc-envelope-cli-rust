use bc_components::URI;
use bc_envelope::{PrivateKeyBase, PublicKeyBase};
use bc_ur::prelude::*;
use bc_xid::{HasPermissions, PrivateKeyOptions, XIDDocument};
use clap::Args;
use anyhow::{bail, Result};

use super::{key_privilege::KeyPrivilege, private_options::PrivateOptions};

/// Create a new XID document from an inception key
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    // Whether to omit, include, or elide the private key.
    #[arg(long, default_value = "include")]
    private: PrivateOptions,

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

impl CommandArgs {
    fn read_keys(&self) -> Result<String> {
        let mut ur_string = String::new();
        if self.keys.is_none() {
            std::io::stdin().read_line(&mut ur_string)?;
        } else {
            ur_string = self.keys.as_ref().unwrap().to_string();
        }
        if ur_string.is_empty() {
            bail!("No public or private key base provided");
        }
        Ok(ur_string.trim().to_string())
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let keys = self.read_keys()?;
        let mut xid_document = if let Ok(private_key_base) = PrivateKeyBase::from_ur_string(&keys) {
            XIDDocument::new_with_private_key(private_key_base)
        } else if let Ok(public_key_base) = PublicKeyBase::from_ur_string(&keys) {
            XIDDocument::new(public_key_base)
        } else {
            bail!("Invalid public or private key base");
        };

        let mut key = xid_document.keys().iter().next().unwrap().clone();
        xid_document.remove_key(&key);

        if !self.endpoints.is_empty() {
            for uri in &self.endpoints {
                key.add_endpoint(uri.clone());
            }
        }
        if !self.permissions.is_empty() {
            key.clear_all_permissions();
            for privilege in &self.permissions {
                key.add_permission((*privilege).into());
            }
        }

        xid_document.add_key(key)?;

        let options = PrivateKeyOptions::from(self.private);
        let unsigned_envelope = xid_document.to_unsigned_envelope_opt(options);
        let ur = UR::new("xid", unsigned_envelope.to_cbor())?;
        Ok(ur.string())
    }
}
