use anyhow::Result;
use bc_components::ARID;
use bc_envelope::prelude::*;
use clap::Args;

/// Generate an Apparently Random Identifer (ARID).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Output ARID in hexadecimal format.
    #[arg(short = 'x', long, default_value = "false")]
    hex: bool,
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let arid = ARID::new();
        if self.hex {
            Ok(hex::encode(arid.as_bytes()))
        } else {
            Ok(arid.ur_string())
        }
    }
}
