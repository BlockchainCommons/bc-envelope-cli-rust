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

/// Generate private keys.
///
/// Derives private keys from a seed, private key base, or generates them
/// randomly. The input can be a ur:seed, ur:envelope, or ur:crypto-prvkey-base.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Optional input from which to derive the private keys.
    /// Can be a seed (ur:seed or ur:envelope) or a private key base
    /// (ur:crypto-prvkey-base). If not provided, generates random private
    /// keys.
    #[arg(name = "INPUT")]
    input: Option<String>,

    /// The signature scheme to use for the signing key.
    #[arg(long, default_value = "schnorr")]
    signing: SigningSchemeArg,

    /// The encapsulation scheme to use for the encryption key.
    #[arg(long, default_value = "x25519")]
    encryption: EncapsulationSchemeArg,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let private_key_base = if let Some(input_ur) = &self.input {
            parse_input(input_ur)?
        } else {
            bc_components::PrivateKeyBase::new()
        };

        let private_keys = match self.signing {
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

fn parse_input(input: &str) -> Result<bc_components::PrivateKeyBase> {
    // Try parsing as PrivateKeyBase first
    if let Ok(private_key_base) =
        bc_components::PrivateKeyBase::from_ur_string(input)
    {
        return Ok(private_key_base);
    }

    // Try parsing as Seed
    if let Ok(seed) = bc_components::Seed::from_ur_string(input) {
        return Ok(bc_components::PrivateKeyBase::new_with_provider(seed));
    }

    // Try parsing as Envelope containing a Seed
    seed_from_envelope(input)
        .map(bc_components::PrivateKeyBase::new_with_provider)
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
