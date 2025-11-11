use bc_xid::XIDVerifySignature;
use clap::{Args, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum VerifyOption {
    #[default]
    /// Do not verify the signature (default).
    None,
    /// Verify that the envelope is signed with the inception key.
    Inception,
}

impl From<VerifyOption> for XIDVerifySignature {
    fn from(opt: VerifyOption) -> Self {
        match opt {
            VerifyOption::None => XIDVerifySignature::None,
            VerifyOption::Inception => XIDVerifySignature::Inception,
        }
    }
}

#[derive(Debug, Args)]
pub struct VerifyArgs {
    /// Signature verification option.
    #[arg(long = "verify", value_enum, default_value = "none")]
    pub verify: VerifyOption,
}

impl Default for VerifyArgs {
    fn default() -> Self { Self { verify: VerifyOption::None } }
}

impl VerifyArgs {
    pub fn verify_signature(&self) -> XIDVerifySignature { self.verify.into() }
}
