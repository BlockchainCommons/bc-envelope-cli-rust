use anyhow::{ bail, Result };
use clap::Args;
use crate::utils::read_password;

use crate::envelope_args::{ EnvelopeArgs, EnvelopeArgsLike };
use bc_components::{ KeyDerivationMethod, PublicKeys, SymmetricKey };
use bc_envelope::prelude::*;

use super::{ASKPASS_HELP, ASKPASS_LONG_HELP};

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
/// If the key is not provided and recipients are provided, an ephemerally-generated key is used.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The symmetric key to use to encrypt the envelope's subject. (ur:crypto-key)
    ///
    /// If not provided and recipients are provided, an ephemerally-generated key is used.
    /// May not be used with the `--password` option.
    #[arg(long, short, conflicts_with = "password")]
    key: Option<String>,

    /// The password to derive the symmetric key.
    ///
    /// If not provided, will be prompted. May not be used with the `--key` option.
    #[arg(long, short, conflicts_with = "key", num_args(0..=1))]
    password: Option<Option<String>>,

    #[arg(long, requires = "password", help = ASKPASS_HELP, long_help = ASKPASS_LONG_HELP)]
    askpass: bool,

    /// The password-based key derivation algorithm to use.
    #[arg(
        long,
        short = 'd',
        value_enum,
        default_value_t = PasswordDerivationType::Argon2id,
        requires = "password"
    )]
    password_derivation: PasswordDerivationType,

    /// The recipients to whom the envelope's subject should be encrypted. (ur:crypto-pubkeys)
    ///
    /// May be provided multiple times.
    #[arg(long, short)]
    recipient: Vec<String>,

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

        // Convert recipients to `PublicKeys`.
        let recipients = self.recipient
            .iter()
            .map(|s| PublicKeys::from_ur_string(s).map_err(anyhow::Error::from))
            .collect::<Result<Vec<PublicKeys>, anyhow::Error>>()?;

        // Get the key
        let key = match self.key {
            Some(ref key) => SymmetricKey::from_ur_string(key)?,
            None => {
                if recipients.is_empty() && self.password.is_none() {
                    bail!("Must provide either a key or recipients, or a password.");
                }
                SymmetricKey::new()
            }
        };

        // Encrypt the subject.
        let mut encrypted_envelope = envelope.encrypt_subject(&key)?;
        // If there are recipients, add them.
        for recipient in recipients {
            encrypted_envelope = encrypted_envelope.add_recipient(&recipient, &key);
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
                &key
            );
        }
        Ok(encrypted_envelope.ur_string())
    }
}
