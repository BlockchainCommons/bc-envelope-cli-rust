use bc_xid::MarkGeneratorOptions;
use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum GeneratorOptions {
    /// Include the mark generator key in plaintext
    #[default]
    Include,

    /// Omit the mark generator
    Omit,

    /// Elide the mark generator (maintains digest tree)
    Elide,

    /// Encrypt the mark generator with a password
    Encrypt,
}

impl GeneratorOptions {
    /// Check if this option requires encryption.
    pub fn is_encrypt(&self) -> bool {
        matches!(self, GeneratorOptions::Encrypt)
    }
}

impl From<GeneratorOptions> for MarkGeneratorOptions {
    fn from(options: GeneratorOptions) -> Self {
        match options {
            GeneratorOptions::Omit => MarkGeneratorOptions::Omit,
            GeneratorOptions::Include => MarkGeneratorOptions::Include,
            GeneratorOptions::Elide => MarkGeneratorOptions::Elide,
            // The Encrypt variant needs additional parameters, so this is a
            // placeholder. Callers should use the password_args module to
            // construct the full MarkGeneratorOptions::Encrypt variant.
            GeneratorOptions::Encrypt => MarkGeneratorOptions::Omit,
        }
    }
}
