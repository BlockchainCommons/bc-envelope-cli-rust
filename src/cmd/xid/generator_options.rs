use bc_xid::XIDGeneratorOptions;
use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum GeneratorOptions {
    /// Include the mark generator key in plaintext
    Include,

    /// Omit the mark generator (no provenance mark will be created)
    #[default]
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

impl From<GeneratorOptions> for XIDGeneratorOptions {
    fn from(options: GeneratorOptions) -> Self {
        match options {
            GeneratorOptions::Omit => XIDGeneratorOptions::Omit,
            GeneratorOptions::Include => XIDGeneratorOptions::Include,
            GeneratorOptions::Elide => XIDGeneratorOptions::Elide,
            // The Encrypt variant needs additional parameters, so this is a
            // placeholder. Callers should use the password_args module to
            // construct the full MarkGeneratorOptions::Encrypt variant.
            GeneratorOptions::Encrypt => XIDGeneratorOptions::Omit,
        }
    }
}
