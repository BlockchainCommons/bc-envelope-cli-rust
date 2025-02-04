use anyhow::{bail, Result};
use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};
use bc_components::{PrivateKeyBase, PublicKeys, SigningPrivateKey, SigningPublicKey, Verifier};
use bc_envelope::prelude::*;

/// Verify a signature on the envelope using the provided verifiers.
///
/// On success, print the original envelope so it can be piped to the next
/// operation. On failure, exit with an error condition.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Don't output the envelope's UR on success.
    #[arg(long, short, default_value = "false")]
    silent: bool,

    /// The minimum number of required valid signatures.
    #[arg(long, short, default_value = "1")]
    threshold: usize,

    /// The verifier(s). May be a private key base (ur:prvkeys), `PublicKeys`
    /// (ur:pubkeys) signing private key (ur:signing-private-key), or a signing
    /// public key (ur:signing-public-key).
    ///
    /// Multiple verifiers may be provided.
    #[arg(long, short)]
    verifier: Vec<String>,

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
        if self.verifier.is_empty() {
            bail!("at least one verifier must be provided");
        }
        let mut private_key_bases: Vec<PrivateKeyBase> = Vec::new();
        let mut public_keys_vec: Vec<PublicKeys> = Vec::new();
        let mut signing_private_keys: Vec<SigningPrivateKey> = Vec::new();
        let mut signing_public_keys: Vec<SigningPublicKey> = Vec::new();
        for v in &self.verifier {
            if let Ok(key) = PrivateKeyBase::from_ur_string(v) {
                private_key_bases.push(key);
            } else if let Ok(key) = PublicKeys::from_ur_string(v) {
                public_keys_vec.push(key);
            } else if let Ok(key) = SigningPrivateKey::from_ur_string(v) {
                signing_private_keys.push(key);
            } else if let Ok(key) = SigningPublicKey::from_ur_string(v) {
                signing_public_keys.push(key);
            } else {
                bail!("invalid verifier: {}", v);
            }
        }
        let mut verifiers: Vec<&dyn Verifier> = Vec::new();
        for key in private_key_bases.iter() {
            verifiers.push(key as &dyn Verifier);
        }
        for key in public_keys_vec.iter() {
            verifiers.push(key as &dyn Verifier);
        }
        for key in signing_private_keys.iter() {
            verifiers.push(key as &dyn Verifier);
        }
        for key in signing_public_keys.iter() {
            verifiers.push(key as &dyn Verifier);
        }
        envelope.clone().verify_signatures_from_threshold(&verifiers, Some(self.threshold))?;
        Ok(if self.silent { "".to_string() } else { envelope.ur_string() })
    }
}
