use anyhow::{Result, bail};
use bc_components::URI;
use bc_envelope::{Envelope, PrivateKeyBase, PublicKeys};
use bc_ur::prelude::*;
use bc_xid::{
    HasNickname, HasPermissions, Key, PrivateKeyOptions, XIDDocument,
};

use super::{
    password_args::{ReadPasswordArgs, WritePasswordArgs},
    private_options::PrivateOptions,
    xid_privilege::XIDPrivilege,
};
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
        key_string = key_string.trim().to_string();
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

    fn read_xid_document_with_password(
        &self,
        password_args: &ReadPasswordArgs,
    ) -> Result<XIDDocument> {
        let envelope = self.read_envelope()?;
        let password = password_args.read_password("Decryption password:")?;
        Ok(XIDDocument::from_unsigned_envelope_with_password(
            &envelope,
            password.as_deref().map(|s| s.as_bytes()),
        )?)
    }
}

/// Get the private key from a key, optionally decrypting it.
///
/// Returns the UR string:
/// - For unencrypted keys: ur:crypto-prvkeys
/// - For encrypted keys without password: ur:envelope of the encrypted envelope
/// - For encrypted keys with correct password: ur:crypto-prvkeys
/// - For encrypted keys with wrong password: Returns an error
pub fn get_private_key_ur(
    key: &Key,
    password_args: &ReadPasswordArgs,
) -> Result<String> {
    use bc_components::PrivateKeys;

    let password = password_args.read_password("Decryption password:")?;

    match key.private_key_envelope(password.as_deref())? {
        None => bail!("No private key present in this key"),
        Some(envelope) => {
            // Try to extract PrivateKeys from the subject
            // If successful, we have decrypted keys - return as ur:crypto-prvkeys
            // If it fails, we have an encrypted envelope - return as ur:envelope
            match PrivateKeys::try_from(envelope.subject()) {
                Ok(private_keys) => {
                    // Successfully extracted PrivateKeys - return the raw UR
                    Ok(private_keys.ur_string())
                }
                Err(_) => {
                    // Subject is not PrivateKeys (it's ENCRYPTED) - return the envelope
                    Ok(envelope.ur_string())
                }
            }
        }
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

/// Convert an XID document to a UR string with password encryption support.
pub fn xid_document_to_ur_string_with_password(
    xid_document: &XIDDocument,
    private_opts: PrivateOptions,
    password_args: &WritePasswordArgs,
) -> Result<String> {
    let options = if private_opts.is_encrypt() {
        // Read the encryption password
        let password = password_args.read_password("Encryption password:")?;
        PrivateKeyOptions::Encrypt {
            method: password_args.method(),
            password: password.into_bytes(),
        }
    } else {
        PrivateKeyOptions::from(private_opts)
    };

    let unsigned_envelope = xid_document.to_unsigned_envelope_opt(options);
    Ok(envelope_to_xid_ur_string(&unsigned_envelope))
}
