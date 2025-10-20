use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

/// Decrypt nodes using provided keys.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Symmetric keys to use for decryption (space-separated UR crypto-keys).
    #[arg(value_delimiter = ' ', required = true)]
    keys: Vec<String>,
}

impl CommandArgs {
    pub fn exec_with_envelope(&self, envelope: Envelope) -> Result<String> {
        use bc_components::SymmetricKey;

        let mut symmetric_keys = Vec::new();
        for ur_string in &self.keys {
            let key = SymmetricKey::from_ur_string(ur_string)?;
            symmetric_keys.push(key);
        }

        let result = envelope.walk_decrypt(&symmetric_keys);
        Ok(result.ur_string())
    }
}
