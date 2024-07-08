use anyhow::{bail, Result};
use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};
use bc_components::{PrivateKeyBase, Signer, SigningOptions, SigningPrivateKey};
use bc_envelope::prelude::*;

use super::generate::HashType;

/// Sign the envelope subject with the provided signer(s).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The signer to sign the envelope subject with. May be a private key base (ur:prvkeys)
    /// or a signing private key (ur:signing-private-key).
    ///
    /// Multiple signers may be provided.
    #[arg(long, short)]
    signer: Vec<String>,

    /// An optional note to add to the envelope.
    #[arg(long)]
    note: Option<String>,

    /// Namespace for SSH signatures.
    #[arg(long, default_value = "envelope")]
    namespace: String,

    /// Hash algorithm for SSH signatures.
    #[arg(long, default_value = "sha256")]
    hash_type: HashType,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;
        if self.signer.is_empty() {
            bail!("at least one signer must be provided");
        }
        let mut private_key_bases: Vec<PrivateKeyBase> = Vec::new();
        let mut signing_private_keys: Vec<SigningPrivateKey> = Vec::new();
        let mut signing_options: Vec<Option<SigningOptions>> = Vec::new();
        for s in &self.signer {
            if let Ok(key) = PrivateKeyBase::from_ur_string(s) {
                private_key_bases.push(key);
            } else if let Ok(key) = SigningPrivateKey::from_ur_string(s) {
                if key.is_ssh() {
                    let namespace = self.namespace.clone();
                    let hash_alg = self.hash_type.to_ssh_hash_alg();
                    signing_options.push(Some(SigningOptions::Ssh {
                        namespace,
                        hash_alg,
                    }));
                } else {
                    signing_options.push(None);
                }
                signing_private_keys.push(key);
            } else {
                bail!("invalid signer: {}", s);
            }
        }
        let mut signers: Vec<(&dyn Signer, Option<SigningOptions>)> = Vec::new();
        for key in private_key_bases.iter() {
            signers.push((key as &dyn Signer, None));
        }
        for i in 0..signing_private_keys.len() {
            signers.push((&signing_private_keys[i] as &dyn Signer, signing_options[i].clone()));
        }
        if let Some(note) = &self.note {
            if signers.len() != 1 {
                bail!("can only add a note on a single signature");
            }
            Ok(envelope.add_signature_opt(signers[0].0, signers[0].1.clone(), Some(note)).ur_string())
        } else {
            Ok(envelope.add_signatures_opt(&signers).ur_string())
        }
    }
}
