use anyhow::{Result, bail};
use bc_components::{PrivateKeys, SigningPrivateKey};
use bc_envelope::Envelope;
use bc_ur::prelude::*;
use bc_xid::XIDSigningOptions;
use clap::{Args, ValueEnum};

use super::password_args::ReadPasswordArgs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum SigningOption {
    /// Do not sign the envelope (default).
    #[default]
    None,
    /// Sign with the XID's inception key (must be available as a signing key).
    Inception,
}

#[derive(Debug, Args)]
pub struct SigningArgs {
    /// Signing option.
    #[arg(long = "sign", value_enum, default_value = "none")]
    pub sign: SigningOption,

    /// The signing key UR (ur:crypto-prvkeys or ur:signing-private-key).
    /// Can be encrypted (ur:envelope). If encrypted, will use the same
    /// password as the XID document.
    #[arg(long = "signing-key", conflicts_with = "sign")]
    pub signing_key: Option<String>,
}

impl Default for SigningArgs {
    fn default() -> Self {
        Self { sign: SigningOption::None, signing_key: None }
    }
}

impl SigningArgs {
    pub fn signing_options(
        &self,
        password_args: Option<&ReadPasswordArgs>,
    ) -> Result<XIDSigningOptions> {
        // If a signing key is provided, use it
        if let Some(key) = &self.signing_key {
            // Try to parse as PrivateKeys first
            if let Ok(private_keys) = PrivateKeys::from_ur_string(key) {
                return Ok(XIDSigningOptions::PrivateKeys(private_keys));
            }

            // Try to parse as SigningPrivateKey
            if let Ok(signing_private_key) =
                SigningPrivateKey::from_ur_string(key)
            {
                return Ok(XIDSigningOptions::SigningPrivateKey(
                    signing_private_key,
                ));
            }

            // If the key string looks like it might be encrypted, try to
            // decrypt it using the same password as the XID document
            if key.starts_with("ur:envelope") {
                let envelope = Envelope::from_ur_string(key)?;

                // Use the same password that unlocks the XID document
                let password = if let Some(pwd_args) = password_args {
                    pwd_args.read_password("Password:")?.ok_or_else(|| {
                        anyhow::anyhow!(
                            "Password required to decrypt signing key"
                        )
                    })?
                } else {
                    bail!(
                        "Encrypted signing key requires password (use --password)"
                    );
                };

                let decrypted_envelope =
                    envelope.unlock_subject(password.as_bytes())?;

                // Try to extract PrivateKeys from the decrypted subject
                if let Ok(private_keys) =
                    decrypted_envelope.extract_subject::<PrivateKeys>()
                {
                    return Ok(XIDSigningOptions::PrivateKeys(private_keys));
                }

                // Try to extract SigningPrivateKey from the decrypted subject
                if let Ok(signing_private_key) =
                    decrypted_envelope.extract_subject::<SigningPrivateKey>()
                {
                    return Ok(XIDSigningOptions::SigningPrivateKey(
                        signing_private_key,
                    ));
                }

                bail!("Decrypted envelope does not contain valid signing keys");
            }

            bail!(
                "Invalid signing key. Expected ur:crypto-prvkeys, ur:signing-private-key, or ur:envelope (encrypted keys)"
            )
        }

        // Otherwise, use the sign option
        match self.sign {
            SigningOption::None => Ok(XIDSigningOptions::None),
            SigningOption::Inception => Ok(XIDSigningOptions::Inception),
        }
    }
}
