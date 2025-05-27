use anyhow::Result;
use bc_components::{KeyDerivationMethod, PublicKeys, SymmetricKey};
use bc_envelope::prelude::*;
use clap::Args;

use super::{ASKPASS_HELP, ASKPASS_LONG_HELP};
use crate::{
    envelope_args::{EnvelopeArgs, EnvelopeArgsLike},
    utils::read_password,
};

/// The password-based key derivation algorithms supported for encryption.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum PasswordDerivationType {
    /// Argon2id key derivation (default)
    Argon2id,
    /// PBKDF2 key derivation
    PBKDF2,
    /// Scrypt key derivation
    Scrypt,
}

impl From<PasswordDerivationType> for KeyDerivationMethod {
    fn from(derivation: PasswordDerivationType) -> Self {
        match derivation {
            PasswordDerivationType::Argon2id => KeyDerivationMethod::Argon2id,
            PasswordDerivationType::PBKDF2 => KeyDerivationMethod::PBKDF2,
            PasswordDerivationType::Scrypt => KeyDerivationMethod::Scrypt,
        }
    }
}

/// Encrypt the envelope's subject using the provided key or password.
///
/// If the content key is not provided, an ephemerally-generated key is used.
///
/// In addition, the content key can be locked, potentially several ways:
///     - If a password is provided, it is used to lock the content key.
///     - If an SSH identity is provided, it is used to lock the content key.
///     - If one or more recipients are provided, the envelope's subject is encrypted
/// for those recipients using the content key.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The content key to use to encrypt the envelope's subject.
    /// (ur:crypto-key)
    ///
    /// If not provided, an ephemerally-generated content key is used.
    #[arg(long, short)]
    key: Option<String>,

    /// A password used to lock the content key.
    ///
    /// If not provided, will be prompted. May not be used with the `--key`
    /// option.
    #[arg(long, short, num_args(0..=1))]
    password: Option<Option<String>>,

    /// Use the `SSH_ASKPASS` environment variable to read the password.
    #[arg(long, requires = "password", help = ASKPASS_HELP, long_help = ASKPASS_LONG_HELP)]
    askpass: bool,

    /// The password-based key derivation algorithm to use with the
    /// `--password` option.
    #[arg(
        long,
        short = 'd',
        value_enum,
        default_value_t = PasswordDerivationType::Argon2id,
        requires = "password"
    )]
    password_derivation: PasswordDerivationType,

    /// The SSH agent key identity used to lock the content key.
    ///
    /// - If provided, the content key will be locked using the Ed25519 key
    /// associated with the specified SSH identity.
    /// - If provided but empty:
    ///     - If multiple keys are available, an error is returned.
    ///     - Otherwise the only Ed25519 key in the SSH agent is used.
    #[arg(long, short)]
    ssh_id: Option<String>,

    /// The recipients to whom the envelope's subject should be encrypted.
    /// (ur:crypto-pubkeys)
    ///
    /// May be provided multiple times.
    #[arg(long, short)]
    recipient: Vec<String>,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> { self.envelope_args.envelope() }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let envelope = self.read_envelope()?;

        // Get the content key
        let content_key = match self.key {
            Some(ref key) => SymmetricKey::from_ur_string(key)?,
            None => SymmetricKey::new(),
        };

        // Encrypt the subject using the content key.
        let mut encrypted_envelope = envelope.encrypt_subject(&content_key)?;

        // Convert recipients to `PublicKeys`.
        let recipients = self
            .recipient
            .iter()
            .map(|s| PublicKeys::from_ur_string(s).map_err(anyhow::Error::from))
            .collect::<Result<Vec<PublicKeys>, anyhow::Error>>()?;

        // If there are recipients, add them.
        for recipient in recipients {
            encrypted_envelope =
                encrypted_envelope.add_recipient(&recipient, &content_key);
        }

        // If there is a password, add it.
        if let Some(password_argument) = &self.password {
            let password = read_password(
                "Encryption password:",
                password_argument.as_ref().map(|s| s.as_str()),
                self.askpass,
            )?;
            encrypted_envelope = encrypted_envelope.add_secret(
                self.password_derivation.into(),
                password.as_bytes(),
                &content_key,
            )?;
        }

        // If there is an SSH ID, add it.
        if let Some(ref ssh_id) = self.ssh_id {
            encrypted_envelope = encrypted_envelope.add_secret(
                KeyDerivationMethod::SSHAgent,
                ssh_id.as_bytes(),
                &content_key,
            )?;
        }

        // Return the encrypted envelope as a UR string.
        Ok(encrypted_envelope.ur_string())
    }
}
