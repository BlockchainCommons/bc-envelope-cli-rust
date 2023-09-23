use bc_envelope::prelude::*;
use clap::Args;

/// Generate a seed.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The number of bytes for the seed. Must be in the range 16..=256.
    #[arg(default_value = "16", long, short, conflicts_with = "hex")]
    count: Option<usize>,

    /// Raw hex data for the seed.
    #[arg(long, short = 'x')]
    hex: Option<String>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let seed;
        if let Some(hex) = &self.hex {
            let bytes = hex::decode(hex)?;
            seed = bc_components::Seed::new_opt(bytes, None, None, None)?;
        } else {
            let count = self.count.unwrap();
            if count < bc_components::Seed::MIN_SEED_LENGTH {
                anyhow::bail!("Seed length must be at least 16 bytes");
            }
            if count > 256 {
                anyhow::bail!("Seed length must be at most 256 bytes");
            }
            seed = bc_components::Seed::new_with_len(count)?;
        }
        Ok(seed.ur_string())
    }
}
