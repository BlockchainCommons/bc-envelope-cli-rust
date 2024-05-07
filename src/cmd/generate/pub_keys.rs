use anyhow::{bail, Result};
use bc_envelope::prelude::*;
use clap::Args;

/// Generate a public key base from a private key base.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The private key base to convert.
    #[arg(name = "PRVKEYS")]
    prv_keys: Option<String>,
}

impl CommandArgs {
    fn read_prv_keys(&self) -> Result<String> {
        let mut ur_string = String::new();
        if self.prv_keys.is_none() {
            std::io::stdin().read_line(&mut ur_string)?;
        } else {
            ur_string = self.prv_keys.as_ref().unwrap().to_string();
        }
        if ur_string.is_empty() {
            bail!("No private key base provided");
        }
        Ok(ur_string.trim().to_string())
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        if let Ok(private_key_base) = bc_components::PrivateKeyBase::from_ur_string(self.read_prv_keys()?) {
            let public_key_base = private_key_base.public_key();
            Ok(public_key_base.ur_string())
        } else {
            bail!("Invalid private key base");
        }
    }
}
