use clap::Args;

use super::{GeneratorOptions, PrivateOptions};

/// Options controlling how sensitive data is output in XID documents.
///
/// This struct provides a unified interface for controlling how private keys
/// and provenance mark generators are handled when outputting XID documents.
/// It can be embedded in any command that needs to output a XID document.
#[derive(Debug, Args, Clone, Default)]
pub struct OutputOptions {
    #[command(flatten)]
    pub private: PrivateOutputArgs,

    #[command(flatten)]
    pub generator: GeneratorOutputArgs,
}

impl OutputOptions {
    /// Create output options with specified private and generator options.
    pub fn new(private: PrivateOptions, generator: GeneratorOptions) -> Self {
        Self {
            private: PrivateOutputArgs { private },
            generator: GeneratorOutputArgs { generator },
        }
    }

    /// Get the private key output option.
    pub fn private_opts(&self) -> PrivateOptions { self.private.private }

    /// Get the generator output option.
    pub fn generator_opts(&self) -> GeneratorOptions {
        self.generator.generator
    }

    /// Check if either private keys or generator need encryption.
    pub fn needs_encryption(&self) -> bool {
        self.private.private.is_encrypt()
            || self.generator.generator.is_encrypt()
    }
}

/// Arguments controlling private key output in XID documents.
#[derive(Debug, Args, Clone, Default)]
pub struct PrivateOutputArgs {
    /// Whether to include, omit, elide, or encrypt private keys.
    ///
    /// - `include`: Include private keys in plaintext
    /// - `omit`: Remove private keys (modifies digest tree, requires
    ///   re-signing)
    /// - `elide`: Replace private keys with ELIDED placeholders (preserves
    ///   digest tree and signature)
    /// - `encrypt`: Encrypt private keys with a password
    #[arg(long = "private", default_value = "include")]
    pub private: PrivateOptions,
}

/// Arguments controlling provenance mark generator output in XID documents.
#[derive(Debug, Args, Clone, Default)]
pub struct GeneratorOutputArgs {
    /// Whether to include, omit, elide, or encrypt the provenance mark
    /// generator.
    ///
    /// - `include`: Include the generator in plaintext
    /// - `omit`: Remove the generator (modifies digest tree, requires
    ///   re-signing)
    /// - `elide`: Replace the generator with ELIDED placeholder (preserves
    ///   digest tree and signature)
    /// - `encrypt`: Encrypt the generator with a password
    ///
    /// Note: For documents without a provenance mark, this option has no
    /// effect.
    #[arg(long = "generator", default_value = "include")]
    pub generator: GeneratorOptions,
}

/// Trait for commands that support output options.
///
/// This trait provides a common interface for commands that need to control
/// how private keys and generators are output in XID documents.
pub trait HasOutputOptions {
    /// Get the output options for this command.
    fn output_options(&self) -> &OutputOptions;
}
