use clap::ValueEnum;
use ssh_key::HashAlg;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum HashType {
    Sha256,
    Sha512,
}

impl HashType {
    pub fn to_ssh_hash_alg(self) -> HashAlg {
        match self {
            Self::Sha256 => HashAlg::Sha256,
            Self::Sha512 => HashAlg::Sha512,
        }
    }
}
