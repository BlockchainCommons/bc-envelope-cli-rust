use anyhow::{Result, bail};
use bc_components::{PrivateKeys, URI};
use bc_envelope::{Envelope, PrivateKeyBase, PublicKeys};
use bc_ur::prelude::*;
use bc_xid::{
    HasNickname, HasPermissions, Key, XIDDocument, XIDGeneratorOptions,
    XIDPrivateKeyOptions, XIDSigningOptions,
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
    PrivateBase(PrivateKeyBase),
    PrivateKeys(PrivateKeys),
    PrivateAndPublicKeys(PrivateKeys, PublicKeys),
}

pub fn read_key(key: Option<&str>) -> Result<InputKey> {
    let key_string = if let Some(key_str) = key {
        key_str.to_string()
    } else {
        // Read from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    };

    if key_string.is_empty() {
        bail!("No key provided");
    }

    // Check if the input contains two space-separated URs
    let parts: Vec<&str> = key_string.split_whitespace().collect();

    if parts.len() == 2 {
        // Try to parse as two separate keys
        if let (Ok(key1), Ok(key2)) =
            (parse_single_key(parts[0]), parse_single_key(parts[1]))
        {
            // Ensure we have exactly one PrivateKeys and one PublicKeys
            match (&key1, &key2) {
                (InputKey::PrivateKeys(prv), InputKey::Public(pub_keys))
                | (InputKey::Public(pub_keys), InputKey::PrivateKeys(prv)) => {
                    return Ok(InputKey::PrivateAndPublicKeys(
                        prv.clone(),
                        pub_keys.clone(),
                    ));
                }
                _ => {
                    bail!(
                        "When providing two keys, one must be crypto-prvkeys and one must be crypto-pubkeys"
                    )
                }
            }
        }
    }

    // Single key or the two-key parse failed - parse as single key
    parse_single_key(&key_string)
}

fn parse_single_key(key_string: &str) -> Result<InputKey> {
    if let Ok(public_keys) = PublicKeys::from_ur_string(key_string) {
        Ok(InputKey::Public(public_keys))
    } else if let Ok(private_key_base) =
        PrivateKeyBase::from_ur_string(key_string)
    {
        Ok(InputKey::PrivateBase(private_key_base))
    } else if let Ok(private_keys) = PrivateKeys::from_ur_string(key_string) {
        Ok(InputKey::PrivateKeys(private_keys))
    } else {
        bail!("Invalid public keys, private key base, or private keys")
    }
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
            // If successful, we have decrypted keys - return as
            // ur:crypto-prvkeys If it fails, we have an encrypted
            // envelope - return as ur:envelope
            match PrivateKeys::try_from(envelope.subject()) {
                Ok(private_keys) => {
                    // Successfully extracted PrivateKeys - return the raw UR
                    Ok(private_keys.ur_string())
                }
                Err(_) => {
                    // Subject is not PrivateKeys (it's ENCRYPTED) - return the
                    // envelope
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
    let options = XIDPrivateKeyOptions::from(private_opts);
    let unsigned_envelope = xid_document
        .to_envelope(
            options,
            XIDGeneratorOptions::default(),
            XIDSigningOptions::default(),
        )
        .unwrap();
    envelope_to_xid_ur_string(&unsigned_envelope)
}

/// Convert an XID document to a UR string with password encryption support.
/// If encrypting private keys, also encrypts the generator with the same password.
pub fn xid_document_to_ur_string_with_password(
    xid_document: &XIDDocument,
    private_opts: PrivateOptions,
    password_args: &WritePasswordArgs,
) -> Result<String> {
    use bc_xid::XIDGeneratorOptions;

    let (private_key_options, generator_options) = if private_opts.is_encrypt()
    {
        // Read the encryption password
        let password = password_args.read_password("Encryption password:")?;
        let private_key_options = XIDPrivateKeyOptions::Encrypt {
            method: password_args.method(),
            password: password.clone().into_bytes(),
        };
        // Encrypt the generator with the same password
        let generator_options = XIDGeneratorOptions::Encrypt {
            method: password_args.method(),
            password: password.into_bytes(),
        };
        (private_key_options, generator_options)
    } else {
        // Include private keys and generator in plaintext based on private_opts
        let private_key_options = XIDPrivateKeyOptions::from(private_opts);
        let generator_options = XIDGeneratorOptions::Include;
        (private_key_options, generator_options)
    };

    let unsigned_envelope = xid_document.to_envelope(
        private_key_options,
        generator_options,
        XIDSigningOptions::default(),
    )?;
    Ok(envelope_to_xid_ur_string(&unsigned_envelope))
}

/// Convert an XID document to a UR string with password encryption and
/// generator options support.
pub fn xid_document_to_ur_string_with_options(
    xid_document: &XIDDocument,
    private_opts: PrivateOptions,
    password_args: &WritePasswordArgs,
    generator_opts: super::generator_options::GeneratorOptions,
    shared_password: Option<String>,
) -> Result<String> {
    use bc_xid::XIDGeneratorOptions;

    let private_key_options = if private_opts.is_encrypt() {
        // Use shared password if available, otherwise read it
        let password = if let Some(ref pwd) = shared_password {
            pwd.clone()
        } else {
            password_args.read_password("Encryption password:")?
        };
        XIDPrivateKeyOptions::Encrypt {
            method: password_args.method(),
            password: password.into_bytes(),
        }
    } else {
        XIDPrivateKeyOptions::from(private_opts)
    };

    let generator_options = if generator_opts.is_encrypt() {
        // Use shared password if available, otherwise read it
        let password = if let Some(ref pwd) = shared_password {
            pwd.clone()
        } else {
            password_args.read_password("Generator password:")?
        };
        XIDGeneratorOptions::Encrypt {
            method: password_args.method(),
            password: password.into_bytes(),
        }
    } else {
        XIDGeneratorOptions::from(generator_opts)
    };

    let unsigned_envelope = xid_document.to_envelope(
        private_key_options,
        generator_options,
        XIDSigningOptions::default(),
    )?;
    Ok(envelope_to_xid_ur_string(&unsigned_envelope))
}
