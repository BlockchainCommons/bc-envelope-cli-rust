use anyhow::{Result, bail};
use bc_envelope::prelude::*;
use clap::Args;

/// Convert private keys to public keys.
///
/// Takes a ur:crypto-prvkeys or ur:signing-private-key and converts it to
/// ur:crypto-pubkeys or ur:signing-public-key.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The private keys to convert (ur:crypto-prvkeys or
    /// ur:signing-private-key). If not provided, reads from stdin.
    #[arg(name = "PRVKEYS")]
    prv_keys: Option<String>,

    /// The comment for SSH public keys.
    #[arg(long, short, default_value = "")]
    comment: String,
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
            bail!("No private keys provided");
        }
        Ok(ur_string.trim().to_string())
    }
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let ur_string = self.read_prv_keys()?;

        // Try to parse as PrivateKeys first
        if let Ok(private_keys) =
            bc_components::PrivateKeys::from_ur_string(&ur_string)
        {
            let mut public_keys = private_keys.public_keys()?;

            // If a comment is provided and the signing key is SSH, update the
            // comment
            if !self.comment.is_empty()
                && let bc_components::SigningPublicKey::SSH(ssh_key) =
                    public_keys.signing_public_key()
            {
                // Create a new SSH key with the updated comment
                let mut new_ssh_key = ssh_key.clone();
                new_ssh_key.set_comment(&self.comment);
                public_keys = bc_components::PublicKeys::new(
                    bc_components::SigningPublicKey::SSH(new_ssh_key),
                    public_keys.enapsulation_public_key().clone(),
                );
            }

            Ok(public_keys.ur_string())
        } else if let Ok(signing_private_key) =
            bc_components::SigningPrivateKey::from_ur_string(&ur_string)
        {
            // Try to derive the public key from the signing private key
            let mut signing_public_key = signing_private_key.public_key()?;

            // If a comment is provided and the signing key is SSH, update the
            // comment
            if !self.comment.is_empty()
                && let bc_components::SigningPublicKey::SSH(ref mut ssh_key) =
                    signing_public_key
            {
                ssh_key.set_comment(&self.comment);
            }

            Ok(signing_public_key.ur_string())
        } else {
            bail!(
                "invalid private key: must be ur:crypto-prvkeys or ur:signing-private-key"
            )
        }
    }
}
