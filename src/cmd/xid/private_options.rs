use bc_xid::PrivateKeyOptions;
use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum PrivateOptions {
    /// Include the private key
    #[default]
    Include,

    /// Omit the private key
    Omit,

    /// Elide the private key
    Elide,
}

impl From<PrivateOptions> for PrivateKeyOptions {
    fn from(options: PrivateOptions) -> Self {
        match options {
            PrivateOptions::Omit => PrivateKeyOptions::Omit,
            PrivateOptions::Include => PrivateKeyOptions::Include,
            PrivateOptions::Elide => PrivateKeyOptions::Elide,
        }
    }
}
