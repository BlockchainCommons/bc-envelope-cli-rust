use anyhow::Result;
use bc_components::{EncapsulationScheme, PrivateKeys, SignatureScheme};
use bc_ur::UREncodable;
use clap::{Args, ValueEnum};

/// Supported signature schemes for keypair generation.
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
    Mldsa44,
    Mldsa65,
    Mldsa87,
}

/// Supported encapsulation schemes for keypair generation.
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "kebab-case")]
enum EncapsulationSchemeArg {
    X25519,
    Mlkem512,
    Mlkem768,
    Mlkem1024,
}

/// Generate keypairs.
///
/// Generates random keypairs, outputting both the private keys
/// (ur:crypto-prvkeys) and public keys (ur:crypto-pubkeys) on the same line
/// separated by a space. Supports post-quantum algorithms (ML-DSA, ML-KEM)
/// that don't support deterministic key derivation.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The signature scheme to use for the signing key.
    #[arg(long, default_value = "schnorr")]
    signing: SigningSchemeArg,

    /// The encapsulation scheme to use for the encryption key.
    #[arg(long, default_value = "x25519")]
    encryption: EncapsulationSchemeArg,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let (signing_private_key, signing_public_key) = match self.signing {
            SigningSchemeArg::Schnorr => SignatureScheme::Schnorr.keypair(),
            SigningSchemeArg::Ecdsa => SignatureScheme::Ecdsa.keypair(),
            SigningSchemeArg::Ed25519 => SignatureScheme::Ed25519.keypair(),
            SigningSchemeArg::SshEd25519 => {
                SignatureScheme::SshEd25519.keypair()
            }
            SigningSchemeArg::SshDsa => SignatureScheme::SshDsa.keypair(),
            SigningSchemeArg::SshEcdsaP256 => {
                SignatureScheme::SshEcdsaP256.keypair()
            }
            SigningSchemeArg::SshEcdsaP384 => {
                SignatureScheme::SshEcdsaP384.keypair()
            }
            SigningSchemeArg::Mldsa44 => SignatureScheme::MLDSA44.keypair(),
            SigningSchemeArg::Mldsa65 => SignatureScheme::MLDSA65.keypair(),
            SigningSchemeArg::Mldsa87 => SignatureScheme::MLDSA87.keypair(),
        };

        let (encapsulation_private_key, encapsulation_public_key) =
            match self.encryption {
                EncapsulationSchemeArg::X25519 => {
                    EncapsulationScheme::X25519.keypair()
                }
                EncapsulationSchemeArg::Mlkem512 => {
                    EncapsulationScheme::MLKEM512.keypair()
                }
                EncapsulationSchemeArg::Mlkem768 => {
                    EncapsulationScheme::MLKEM768.keypair()
                }
                EncapsulationSchemeArg::Mlkem1024 => {
                    EncapsulationScheme::MLKEM1024.keypair()
                }
            };

        let private_keys = PrivateKeys::with_keys(
            signing_private_key,
            encapsulation_private_key,
        );
        let public_keys = bc_components::PublicKeys::new(
            signing_public_key,
            encapsulation_public_key,
        );

        Ok(format!(
            "{} {}",
            private_keys.ur_string(),
            public_keys.ur_string()
        ))
    }
}
