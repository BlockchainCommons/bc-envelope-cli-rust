use bc_components::Nonce;
use bc_ur::UREncodable;
use clap::Args;

/// Generate a Number Used Once (Nonce).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let nonce = Nonce::new();
        Ok(nonce.ur_string())
    }
}
