use bc_components::SymmetricKey;
use clap::Args;
use bc_envelope::prelude::*;
use anyhow::Result;

/// Generate a symmetric encryption key.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let key = SymmetricKey::new();
        Ok(key.ur_string())
    }
}
