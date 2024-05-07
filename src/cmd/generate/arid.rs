use bc_components::ARID;
use clap::Args;
use bc_envelope::prelude::*;
use anyhow::Result;

/// Generate an Apparently Random Identifer (ARID).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Output ARID in hexadecimal format.
    #[arg(short='x', long, default_value="false")]
    hex: bool,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let arid = ARID::new();
        if self.hex {
            Ok(hex::encode(arid.data()))
        } else {
            Ok(arid.ur_string())
        }
    }
}
