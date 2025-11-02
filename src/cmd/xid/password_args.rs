use anyhow::Result;
use bc_components::KeyDerivationMethod;
use clap::Args;

use crate::{
    cmd::{ASKPASS_HELP, ASKPASS_LONG_HELP},
    utils::read_password,
};

/// Password derivation method for encrypting/decrypting XID private keys.
#[derive(Debug, Clone, Copy, clap::ValueEnum, Default)]
pub enum PasswordMethod {
    /// Argon2id key derivation (recommended, default)
    #[default]
    Argon2id,
    /// PBKDF2 key derivation
    PBKDF2,
    /// Scrypt key derivation
    Scrypt,
}

impl From<PasswordMethod> for KeyDerivationMethod {
    fn from(method: PasswordMethod) -> Self {
        match method {
            PasswordMethod::Argon2id => KeyDerivationMethod::Argon2id,
            PasswordMethod::PBKDF2 => KeyDerivationMethod::PBKDF2,
            PasswordMethod::Scrypt => KeyDerivationMethod::Scrypt,
        }
    }
}

/// Arguments for reading an encrypted XID document.
///
/// Use this when a command needs to load an XID document that may have
/// encrypted private keys.
#[derive(Debug, Args)]
#[group(skip)]
pub struct ReadPasswordArgs {
    /// The password to decrypt private keys in the XID document.
    ///
    /// If the document has encrypted private keys and no password is provided,
    /// the document will be loaded without the private key material.
    #[arg(long, num_args(0..=1), value_name = "PASSWORD")]
    pub password: Option<Option<String>>,

    /// Use the `SSH_ASKPASS` environment variable to read the password.
    #[arg(long, requires = "password", help = ASKPASS_HELP, long_help = ASKPASS_LONG_HELP)]
    pub askpass: bool,
}

impl ReadPasswordArgs {
    /// Read the password from the arguments or prompt the user.
    ///
    /// Returns `None` if no password was specified (allowing the document to
    /// be loaded without decrypting private keys).
    pub fn read_password(&self, prompt: &str) -> Result<Option<String>> {
        match &self.password {
            Some(password_arg) => {
                let password = read_password(
                    prompt,
                    password_arg.as_deref(),
                    self.askpass,
                )?;
                Ok(Some(password))
            }
            None => Ok(None),
        }
    }

    /// Check if password arguments were provided.
    #[allow(dead_code)]
    pub fn has_password(&self) -> bool { self.password.is_some() }
}

/// Arguments for writing an encrypted XID document.
///
/// Use this when a command needs to save an XID document with encrypted
/// private keys.
#[derive(Debug, Args)]
#[group(skip)]
pub struct WritePasswordArgs {
    /// The password to encrypt private keys in the XID document.
    ///
    /// If not provided, will be prompted when `--private encrypt` is used.
    #[arg(long = "encrypt-password", num_args(0..=1), value_name = "PASSWORD")]
    pub encrypt_password: Option<Option<String>>,

    /// Use the `SSH_ASKPASS` environment variable to read the encryption
    /// password.
    #[arg(long = "encrypt-askpass", requires = "encrypt_password", help = ASKPASS_HELP, long_help = ASKPASS_LONG_HELP)]
    pub encrypt_askpass: bool,

    /// The key derivation method to use when encrypting private keys.
    #[arg(
        long = "encrypt-method",
        value_enum,
        default_value_t = PasswordMethod::Argon2id,
        requires = "encrypt_password"
    )]
    pub encrypt_method: PasswordMethod,
}

impl WritePasswordArgs {
    /// Read the encryption password from the arguments or prompt the user.
    ///
    /// This should only be called when encrypting private keys.
    pub fn read_password(&self, prompt: &str) -> Result<String> {
        read_password(
            prompt,
            self.encrypt_password.as_ref().and_then(|p| p.as_deref()),
            self.encrypt_askpass,
        )
    }

    /// Check if encryption password arguments were provided.
    #[allow(dead_code)]
    pub fn has_password(&self) -> bool { self.encrypt_password.is_some() }

    /// Get the key derivation method.
    pub fn method(&self) -> KeyDerivationMethod { self.encrypt_method.into() }
}

/// Combined arguments for reading and writing encrypted XID documents.
///
/// Use this when a command needs to both load an encrypted document and save
/// it (potentially with different encryption).
#[derive(Debug, Args)]
#[group(skip)]
pub struct ReadWritePasswordArgs {
    #[command(flatten)]
    pub read: ReadPasswordArgs,

    #[command(flatten)]
    pub write: WritePasswordArgs,
}

impl ReadWritePasswordArgs {
    /// Read the decryption password.
    #[allow(dead_code)]
    pub fn read_password(&self) -> Result<Option<String>> {
        self.read.read_password("Decryption password:")
    }

    /// Read the encryption password.
    #[allow(dead_code)]
    pub fn read_encrypt_password(&self) -> Result<String> {
        self.write.read_password("Encryption password:")
    }

    /// Check if decryption password was provided.
    #[allow(dead_code)]
    pub fn has_read_password(&self) -> bool { self.read.has_password() }

    /// Check if encryption password was provided.
    #[allow(dead_code)]
    pub fn has_write_password(&self) -> bool { self.write.has_password() }

    /// Get the encryption method.
    #[allow(dead_code)]
    pub fn encrypt_method(&self) -> KeyDerivationMethod { self.write.method() }
}
