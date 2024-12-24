use bc_components::URI;
use bc_envelope::{ PrivateKeyBase, PublicKeyBase };
use bc_ur::prelude::*;

use anyhow::{ Result, bail };
use bc_xid::{ HasName, HasPermissions, Key, XIDDocument };

use crate::envelope_args::EnvelopeArgsLike;

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

pub fn read_public_key(key: Option<&str>) -> Result<PublicKeyBase> {
    let key = read_key(key)?;
    match key {
        InputKey::Public(public_key_base) => Ok(public_key_base),
        _ => bail!("Expected a public key, but found a private key."),
    }
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

pub trait XIDDocumentReadable: EnvelopeArgsLike {
    fn read_xid_document(&self) -> Result<XIDDocument> {
        let envelope = self.read_envelope()?;
        XIDDocument::from_unsigned_envelope(&envelope)
    }
}
