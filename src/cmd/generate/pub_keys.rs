use bc_envelope::prelude::*;
use clap::Args;

/// Generate a public key base from a private key base.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The private key base to convert.
    #[arg(name = "PRVKEYS")]
    prv_keys: String,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        if let Ok(private_key_base) = bc_components::PrivateKeyBase::from_ur_string(&self.prv_keys) {
            let public_key_base = private_key_base.public_keys();
            Ok(public_key_base.ur_string())
        } else {
            anyhow::bail!("Invalid private key base");
        }
    }
}
