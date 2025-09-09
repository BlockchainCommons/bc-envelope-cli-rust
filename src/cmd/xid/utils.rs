use anyhow::{Result, bail};
use bc_components::URI;
use bc_envelope::{Envelope, PrivateKeyBase, PublicKeys};
use bc_ur::prelude::*;
use bc_xid::{
    HasNickname, HasPermissions, Key, PrivateKeyOptions, XIDDocument,
};

use super::{private_options::PrivateOptions, xid_privilege::XIDPrivilege};
use crate::envelope_args::EnvelopeArgsLike;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputKey {
    Public(PublicKeys),
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
    let input_key =
        if let Ok(public_keys) = PublicKeys::from_ur_string(&key_string) {
            InputKey::Public(public_keys)
        } else if let Ok(private_key_base) =
            PrivateKeyBase::from_ur_string(&key_string)
        {
            InputKey::Private(private_key_base)
        } else {
            bail!("Invalid public or private key base");
        };
    Ok(input_key)
}

pub fn read_public_key(key: Option<&str>) -> Result<PublicKeys> {
    let key = read_key(key)?;
    match key {
        InputKey::Public(public_keys) => Ok(public_keys),
        _ => bail!("Expected a public key, but found a private key."),
    }
}

pub fn update_key(
    key: &mut Key,
    nickname: &str,
    endpoints: &[URI],
    permissions: &[XIDPrivilege],
) {
    if !nickname.is_empty() {
        key.set_nickname(nickname);
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
        Ok(XIDDocument::from_unsigned_envelope(&envelope)?)
    }
}

pub fn read_uri(uri: Option<&URI>) -> Result<URI> {
    let mut uri_string = String::new();
    if uri.is_none() {
        std::io::stdin().read_line(&mut uri_string)?;
    } else {
        uri_string = uri.as_ref().unwrap().to_string();
    }
    if uri_string.is_empty() {
        bail!("No URI provided");
    }
    Ok(URI::new(uri_string.trim())?)
}

pub fn envelope_to_xid_ur_string(envelope: &Envelope) -> String {
    UR::new("xid", envelope.to_cbor()).unwrap().string()
}

pub fn xid_document_to_ur_string(
    xid_document: &XIDDocument,
    private_opts: PrivateOptions,
) -> String {
    let options = PrivateKeyOptions::from(private_opts);
    let unsigned_envelope = xid_document.to_unsigned_envelope_opt(options);
    envelope_to_xid_ur_string(&unsigned_envelope)
}
