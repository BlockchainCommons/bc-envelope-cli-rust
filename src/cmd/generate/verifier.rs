use anyhow::{Result, bail};
use bc_envelope::prelude::*;
use clap::Args;

use super::SignerType;

/// Generate signing public key from a signing private key, `PublicKeys`, or
/// private key base.
///
/// Note that a private key base will always produce a Schnorr signing public
/// key. A signing private key or `PublicKeys` will produce the corresponding
/// signing public key.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The private key base or signing private key to convert.
    #[arg(name = "PRVKEY")]
    prv_key: Option<String>,
}

impl CommandArgs {
    fn read_prv_key(&self) -> Result<String> {
        let mut ur_string = String::new();
        if self.prv_key.is_none() {
            std::io::stdin().read_line(&mut ur_string)?;
        } else {
            ur_string = self.prv_key.as_ref().unwrap().to_string();
        }
        if ur_string.is_empty() {
            bail!("No signing private key or private key base provided");
        }
        Ok(ur_string.trim().to_string())
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let ur_string = self.read_prv_key()?;
        if let Ok(private_key_base) =
            bc_components::PrivateKeyBase::from_ur_string(&ur_string)
        {
            let signing_private_key = SignerType::Schnorr
                .to_signing_private_key(&private_key_base, "")?;
            Ok(signing_private_key.ur_string())
        } else if let Ok(public_keys) =
            bc_components::PublicKeys::from_ur_string(&ur_string)
        {
            Ok(public_keys.signing_public_key().ur_string())
        } else if let Ok(signing_private_key) =
            bc_components::SigningPrivateKey::from_ur_string(&ur_string)
        {
            let signing_public_key = signing_private_key.public_key()?;
            Ok(signing_public_key.ur_string())
        } else {
            bail!("Invalid signing private key or private key base");
        }
    }
}
