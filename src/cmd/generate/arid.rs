use bc_components::ARID;
use clap::Args;
use bc_envelope::prelude::*;

/// Generate an Apparently Random Identifer (ARID).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        Ok(ARID::new().ur_string())
    }
}
