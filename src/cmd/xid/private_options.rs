use bc_xid::PrivateKeyOptions;
use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum PrivateOptions {
    /// Include the private key in plaintext
    #[default]
    Include,

    /// Omit the private key
    Omit,

    /// Elide the private key (maintains digest tree)
    Elide,

    /// Encrypt the private key with a password
    Encrypt,
}

impl PrivateOptions {
    /// Check if this option requires encryption.
    pub fn is_encrypt(&self) -> bool { matches!(self, PrivateOptions::Encrypt) }
}

impl From<PrivateOptions> for PrivateKeyOptions {
    fn from(options: PrivateOptions) -> Self {
        match options {
            PrivateOptions::Omit => PrivateKeyOptions::Omit,
            PrivateOptions::Include => PrivateKeyOptions::Include,
            PrivateOptions::Elide => PrivateKeyOptions::Elide,
            // The Encrypt variant needs additional parameters, so this is a
            // placeholder. Callers should use the password_args module to
            // construct the full PrivateKeyOptions::Encrypt variant.
            PrivateOptions::Encrypt => PrivateKeyOptions::Omit,
        }
    }
}
