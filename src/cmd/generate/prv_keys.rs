use bc_envelope::prelude::*;
use clap::Args;

/// Generate a private key base.
///
/// Generated randomly, or deterministically if a seed is provided.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The seed from which to derive the private key base (ur:crypto-seed).
    #[arg(long, short)]
    seed: Option<String>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        if let Some(seed_ur) = &self.seed {
            let seed = bc_components::Seed::from_ur_string(seed_ur)?;
            let private_key_base = bc_components::PrivateKeyBase::new_with_provider(seed);
            Ok(private_key_base.ur_string())
        } else {
            let private_key_base = bc_components::PrivateKeyBase::new();
            Ok(private_key_base.ur_string())
        }
    }
}
