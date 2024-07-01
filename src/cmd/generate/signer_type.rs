use bc_components::{PrivateKeyBase, PublicKeyBase, SigningPrivateKey};
use clap::ValueEnum;
use ssh_key::{Algorithm as SSHAlgorithm, EcdsaCurve, HashAlg};
use anyhow::Result;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SignerType {
    /// Schnorr
    Schnorr,

    /// ECDSA
    Ecdsa,

    /// SSH-Ed25519
    SshEd25519,

    /// SSH-RSA SHA-256
    SshRsaSha256,

    /// SSH-RSA SHA-512
    SshRsaSha512,

    /// SSH-DSA
    SshDsa,

    /// SSH-ECDSA NIST P-256
    SshEcdsaP256,

    /// SSH-ECDSA NIST P-384
    SshEcdsaP384,

    /// SSH-ECDSA NIST P-521
    SshEcdsaP521,
}

impl SignerType {
    pub fn to_signing_private_key(self, private_key_base: &PrivateKeyBase, ssh_comment: impl Into<String>) -> Result<SigningPrivateKey> {
        match self {
            Self::Schnorr => Ok(private_key_base.schnorr_signing_private_key()),
            Self::Ecdsa => Ok(private_key_base.ecdsa_signing_private_key()),
            Self::SshEd25519 => private_key_base.ssh_signing_private_key(SSHAlgorithm::Ed25519, ssh_comment),
            Self::SshRsaSha256 => private_key_base.ssh_signing_private_key(SSHAlgorithm::Rsa { hash: Some(HashAlg::Sha256) }, ssh_comment),
            Self::SshRsaSha512 => private_key_base.ssh_signing_private_key(SSHAlgorithm::Rsa { hash: Some(HashAlg::Sha512) }, ssh_comment),
            Self::SshDsa => private_key_base.ssh_signing_private_key(SSHAlgorithm::Dsa, ssh_comment),
            Self::SshEcdsaP256 => private_key_base.ssh_signing_private_key(SSHAlgorithm::Ecdsa { curve: EcdsaCurve::NistP256 }, ssh_comment),
            Self::SshEcdsaP384 => private_key_base.ssh_signing_private_key(SSHAlgorithm::Ecdsa { curve: EcdsaCurve::NistP384 }, ssh_comment),
            Self::SshEcdsaP521 => private_key_base.ssh_signing_private_key(SSHAlgorithm::Ecdsa { curve: EcdsaCurve::NistP521 }, ssh_comment),
        }
    }

    pub fn to_public_key_base(self, private_key_base: &PrivateKeyBase, ssh_comment: impl Into<String>) -> Result<PublicKeyBase> {
        let signing_private_key = self.to_signing_private_key(private_key_base, ssh_comment)?;
        Ok(PublicKeyBase::new(
            signing_private_key.public_key(),
            private_key_base.agreement_private_key().public_key()
        ))
    }
}
