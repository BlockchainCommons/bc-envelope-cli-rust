use anyhow::Result;
use bc_envelope::prelude::*;
use clap::{Args, ValueEnum};
use dcbor::prelude::Date;

/// Supported signature schemes for private key generation.
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "kebab-case")]
enum SigningSchemeArg {
    Schnorr,
    Ecdsa,
    Ed25519,
    SshEd25519,
    SshDsa,
    SshEcdsaP256,
    SshEcdsaP384,
}

/// Supported encapsulation schemes for private key generation.
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "kebab-case")]
enum EncapsulationSchemeArg {
    X25519,
}

/// Generate a private key base.
///
/// Generated randomly, or deterministically if a seed is provided.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The seed from which to derive the private key base (ur:seed or
    /// ur:envelope).
    #[arg(long, short)]
    seed: Option<String>,

    /// The signature scheme to use for the signing key.
    #[arg(long)]
    signing: Option<SigningSchemeArg>,

    /// The encapsulation scheme to use for the encryption key.
    #[arg(long)]
    encryption: Option<EncapsulationSchemeArg>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        // If either signing or encryption scheme is specified, generate
        // PrivateKeys
        if self.signing.is_some() || self.encryption.is_some() {
            self.generate_private_keys()
        } else {
            // Otherwise, generate PrivateKeyBase (legacy behavior)
            self.generate_private_key_base()
        }
    }
}

impl CommandArgs {
    fn generate_private_key_base(&self) -> Result<String> {
        if let Some(seed_ur) = &self.seed {
            let seed = parse_seed_input(seed_ur)?;
            let private_key_base =
                bc_components::PrivateKeyBase::new_with_provider(seed);
            Ok(private_key_base.ur_string())
        } else {
            let private_key_base = bc_components::PrivateKeyBase::new();
            Ok(private_key_base.ur_string())
        }
    }

    fn generate_private_keys(&self) -> Result<String> {
        let private_key_base = if let Some(seed_ur) = &self.seed {
            let seed = parse_seed_input(seed_ur)?;
            bc_components::PrivateKeyBase::new_with_provider(seed)
        } else {
            bc_components::PrivateKeyBase::new()
        };

        // Determine the signing scheme to use (default to Schnorr)
        let signing_scheme = self.signing.unwrap_or(SigningSchemeArg::Schnorr);

        let private_keys = match signing_scheme {
            SigningSchemeArg::Schnorr => {
                private_key_base.schnorr_private_keys()
            }
            SigningSchemeArg::Ecdsa => private_key_base.ecdsa_private_keys(),
            SigningSchemeArg::Ed25519 => bc_components::PrivateKeys::with_keys(
                private_key_base.ed25519_signing_private_key(),
                bc_components::EncapsulationPrivateKey::X25519(
                    private_key_base.x25519_private_key(),
                ),
            ),
            SigningSchemeArg::SshEd25519 => private_key_base
                .ssh_private_keys(ssh_key::Algorithm::Ed25519, "")?,
            SigningSchemeArg::SshDsa => private_key_base
                .ssh_private_keys(ssh_key::Algorithm::Dsa, "")?,
            SigningSchemeArg::SshEcdsaP256 => private_key_base
                .ssh_private_keys(
                    ssh_key::Algorithm::Ecdsa {
                        curve: ssh_key::EcdsaCurve::NistP256,
                    },
                    "",
                )?,
            SigningSchemeArg::SshEcdsaP384 => private_key_base
                .ssh_private_keys(
                    ssh_key::Algorithm::Ecdsa {
                        curve: ssh_key::EcdsaCurve::NistP384,
                    },
                    "",
                )?,
        };

        Ok(private_keys.ur_string())
    }
}

fn parse_seed_input(input: &str) -> Result<bc_components::Seed> {
    match bc_components::Seed::from_ur_string(input) {
        Ok(seed) => Ok(seed),
        Err(_) => seed_from_envelope(input),
    }
}

fn seed_from_envelope(input: &str) -> Result<bc_components::Seed> {
    let envelope = Envelope::from_ur_string(input)?;
    envelope.check_type(&known_values::SEED_TYPE)?;

    let data = envelope
        .subject()
        .try_leaf()?
        .try_into_byte_string()?
        .to_vec();
    let name = envelope
        .extract_optional_object_for_predicate::<String>(known_values::NAME)?
        .unwrap_or_default();
    let note = envelope
        .extract_optional_object_for_predicate::<String>(known_values::NOTE)?
        .unwrap_or_default();
    let creation_date = envelope
        .extract_optional_object_for_predicate::<Date>(known_values::DATE)?
        .map(|date| date.as_ref().clone());

    bc_components::Seed::new_opt(
        data,
        optional_non_empty(name),
        optional_non_empty(note),
        creation_date,
    )
    .map_err(Into::into)
}

fn optional_non_empty(value: String) -> Option<String> {
    if value.is_empty() { None } else { Some(value) }
}
