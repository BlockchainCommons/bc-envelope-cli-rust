use anyhow::{bail, Result};
use clap::Args;
use ssh_key::{public::KeyData, HashAlg};

use crate::utils::read_argument;
use bc_components::{PrivateKeyBase, PublicKeyBase, Seed, Signature, SigningPrivateKey, SigningPublicKey};
use bc_envelope::prelude::*;

/// Provide type and other information about the object.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The object to provide information for. If not provided, the object will be read from stdin.
    object: Option<String>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let mut result = Vec::<String>::new();

        fn add(result: &mut Vec<String>, field: impl Into<String>, value: impl Into<String>) {
            result.push(format!("{}: {}", field.into(), value.into()));
        }

        fn add_public_key_info(result: &mut Vec<String>, public_key: &KeyData) {
            add(result, "Algorithm", format!("{}", public_key.algorithm()));
            let fingerprint = public_key.fingerprint(HashAlg::default());
            add(result, "Fingerprint", format!("{}", &fingerprint));
            let algorithm_str = public_key.algorithm().to_string();
            let algorithm_formatted = algorithm_str.strip_prefix("ssh-").unwrap_or(&algorithm_str);
            result.push(fingerprint.to_randomart(&format!("[{}]", algorithm_formatted).to_uppercase()));
        }

        let object = read_argument(self.object.as_deref())?;
        if object.trim().strip_prefix("ur:").is_some() {
            let ur = UR::from_ur_string(&object)?;
            let ur_type = ur.ur_type_str();
            add(&mut result, "Format", format!("ur:{}", ur_type));
            match ur_type {
                "envelope" => {
                    let _envelope = Envelope::from_ur(&ur)?;
                    add(&mut result, "Description", "Gordian Envelope");
                }
                "seed" => {
                    let _seed = Seed::from_ur(&ur)?;
                    add(&mut result, "Description", "Cryptographic Seed");
                }
                "prvkeys" => {
                    let _private_key_base = PrivateKeyBase::from_ur(&ur)?;
                    add(&mut result, "Description", "Private Key Base");
                }
                "pubkeys" => {
                    let _public_key_base = PublicKeyBase::from_ur(&ur)?;
                    add(&mut result, "Description", "Public Key Base");
                }
                "signing-private-key" => {
                    let signing_private_key = SigningPrivateKey::from_ur(&ur)?;
                    match signing_private_key {
                        SigningPrivateKey::Schnorr(_) => add(&mut result, "Description", "Schnorr Signing Private Key"),
                        SigningPrivateKey::ECDSA(_) => add(&mut result, "Description", "ECDSA Signing Private Key"),
                        SigningPrivateKey::Ed25519(_) => add(&mut result, "Description", "Ed25519 Signing Private Key"),
                        SigningPrivateKey::Dilithium(dilithium_key) => add(&mut result, "Description", format!("{:?} Signing Private Key", dilithium_key.level())),
                        SigningPrivateKey::SSH(ssh_key) => {
                            add(&mut result, "Description", "SSH Signing Private Key");
                            add_public_key_info(&mut result, ssh_key.public_key().key_data());
                        }
                    };
                }
                "signing-public-key" => {
                    let signing_public_key = SigningPublicKey::from_ur(&ur)?;
                    match signing_public_key {
                        SigningPublicKey::Schnorr(_) => add(&mut result, "Description", "Schnorr Signing Public Key"),
                        SigningPublicKey::ECDSA(_) => add(&mut result, "Description", "ECDSA Signing Public Key"),
                        SigningPublicKey::Ed25519(_) => add(&mut result, "Description", "Ed25519 Signing Public Key"),
                        SigningPublicKey::Dilithium(dilithium_key) => add(&mut result, "Description", format!("{:?} Signing Public Key", dilithium_key.level())),
                        SigningPublicKey::SSH(ssh_key) => {
                            add(&mut result, "Description", "SSH Signing Public Key");
                            add_public_key_info(&mut result, ssh_key.key_data());
                        }
                    };
                }
                "signature" => {
                    let signature = Signature::from_ur(&ur)?;
                    match signature {
                        Signature::Schnorr { .. } => add(&mut result, "Description", "Schnorr Signature"),
                        Signature::ECDSA(_) => add(&mut result, "Description", "ECDSA Signature"),
                        Signature::Ed25519(_) => add(&mut result, "Description", "Ed25519 Signature"),
                        Signature::Dilithium(_) => add(&mut result, "Description", "Dilithium Signature"),
                        Signature::SSH(ssh_sig) => {
                            add(&mut result, "Description", "SSH Signature");
                            add(&mut result, "Namespace", ssh_sig.namespace().to_string());
                            add_public_key_info(&mut result, ssh_sig.public_key());
                        },
                    };
                }
                _ => {
                    bail!("Unknown UR type: {}", ur_type);
                }
            }
        } else {
            bail!("Unknown object.");
        }
        Ok(result.join("\n"))
    }
}
