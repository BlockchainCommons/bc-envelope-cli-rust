use anyhow::{Result, bail};
use bc_components::{
    EncapsulationPublicKey, PrivateKeyBase, PublicKeys, SigningPrivateKey,
};
use clap::ValueEnum;
use ssh_key::{Algorithm as SSHAlgorithm, EcdsaCurve, HashAlg};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SignerType {
    /// Schnorr
    Schnorr,

    /// ECDSA
    Ecdsa,

    /// Ed25519
    Ed25519,

    /// SSH-Ed25519
    SshEd25519,

    /// MLDSA44
    MLDSA44,

    /// MLDSA65
    MLDSA65,

    /// MLDSA87
    MLDSA87,

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
    // Disabled due to a bug in the ssh-key crate.
    // See: https://github.com/RustCrypto/SSH/issues/232

    // SSH-ECDSA NIST P-521
    // SshEcdsaP521,
}

impl SignerType {
    pub fn to_signing_private_key(
        self,
        private_key_base: &PrivateKeyBase,
        ssh_comment: impl Into<String>,
    ) -> Result<SigningPrivateKey> {
        match self {
            Self::Schnorr => Ok(private_key_base.schnorr_signing_private_key()),
            Self::Ecdsa => Ok(private_key_base.ecdsa_signing_private_key()),
            Self::Ed25519 => Ok(private_key_base.ed25519_signing_private_key()),

            Self::MLDSA44 => bail!("MLDSA conversion not supported"),
            Self::MLDSA65 => bail!("MLDSA conversion not supported"),
            Self::MLDSA87 => bail!("MLDSA conversion not supported"),

            Self::SshEd25519 => Ok(private_key_base
                .ssh_signing_private_key(SSHAlgorithm::Ed25519, ssh_comment)?),
            Self::SshRsaSha256 => Ok(private_key_base.ssh_signing_private_key(
                SSHAlgorithm::Rsa { hash: Some(HashAlg::Sha256) },
                ssh_comment,
            )?),
            Self::SshRsaSha512 => Ok(private_key_base.ssh_signing_private_key(
                SSHAlgorithm::Rsa { hash: Some(HashAlg::Sha512) },
                ssh_comment,
            )?),
            Self::SshDsa => Ok(private_key_base
                .ssh_signing_private_key(SSHAlgorithm::Dsa, ssh_comment)?),
            Self::SshEcdsaP256 => Ok(private_key_base.ssh_signing_private_key(
                SSHAlgorithm::Ecdsa { curve: EcdsaCurve::NistP256 },
                ssh_comment,
            )?),
            Self::SshEcdsaP384 => Ok(private_key_base.ssh_signing_private_key(
                SSHAlgorithm::Ecdsa { curve: EcdsaCurve::NistP384 },
                ssh_comment,
            )?),
            // Self::SshEcdsaP521 =>
            // private_key_base.ssh_signing_private_key(SSHAlgorithm::Ecdsa {
            // curve: EcdsaCurve::NistP521 }, ssh_comment),
        }
    }

    pub fn to_public_keys(
        self,
        private_key_base: &PrivateKeyBase,
        ssh_comment: impl Into<String>,
    ) -> Result<PublicKeys> {
        let signing_private_key =
            self.to_signing_private_key(private_key_base, ssh_comment)?;
        Ok(PublicKeys::new(
            signing_private_key.public_key()?,
            EncapsulationPublicKey::X25519(
                private_key_base.x25519_private_key().public_key(),
            ),
        ))
    }
}
