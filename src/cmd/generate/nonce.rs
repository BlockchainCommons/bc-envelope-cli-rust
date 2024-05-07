use bc_components::Nonce;
use bc_envelope::prelude::*;
use clap::Args;
use anyhow::Result;

/// Generate a Number Used Once (Nonce).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let nonce = Nonce::new();
        Ok(nonce.ur_string())
    }
}
