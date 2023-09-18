use bc_components::SymmetricKey;
use bc_ur::UREncodable;
use clap::Args;

/// Generate a symmetric encryption key.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let key = SymmetricKey::new();
        Ok(key.ur_string())
    }
}
