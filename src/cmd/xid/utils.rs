use bc_components::URI;
use bc_envelope::{PrivateKeyBase, PublicKeyBase};
use bc_ur::prelude::*;

use anyhow::{Result, bail};
use bc_xid::{HasPermissions, Key};

use super::key_privilege::KeyPrivilege;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputKey {
    Public(PublicKeyBase),
    Private(PrivateKeyBase),
}

pub fn read_key(key: Option<&str>) -> Result<InputKey> {
    let mut key_string = String::new();
    if key.is_none() {
        std::io::stdin().read_line(&mut key_string)?;
    } else {
        key_string = key.as_ref().unwrap().to_string();
    }
    if key_string.is_empty() {
        bail!("No key provided");
    }
    let input_key = if let Ok(public_key_base) = PublicKeyBase::from_ur_string(&key_string) {
        InputKey::Public(public_key_base)
    } else if let Ok(private_key_base) = PrivateKeyBase::from_ur_string(&key_string) {
        InputKey::Private(private_key_base)
    } else {
        bail!("Invalid public or private key base");
    };
    Ok(input_key)
}

pub fn update_key(key: &mut Key, name: &str, endpoints: &[URI], permissions: &[KeyPrivilege]) {
    if !name.is_empty() {
        key.set_name(name);
    }
    
    if !endpoints.is_empty() {
        for uri in endpoints {
            key.add_endpoint(uri.clone());
        }
    }
    if !permissions.is_empty() {
        key.clear_all_permissions();
        for privilege in permissions {
            key.add_permission((*privilege).into());
        }
    }
}
