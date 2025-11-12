use anyhow::{Result, bail};
use bc_components::{
    PublicKeys, Signature, SigningPrivateKey, SigningPublicKey,
};
use bc_envelope::prelude::*;
use clap::Args;
use ssh_key::LineEnding;

use super::{ASKPASS_HELP, ASKPASS_LONG_HELP};
use crate::{read_argument, read_password};

/// Import the given object to UR form.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The UR to be exported.
    ///
    /// May be:
    ///
    /// - An SSH `ur:signing-private-key` (exported to PEM),
    ///
    /// - An SSH `ur:signing-public-key` (exported to single-line text),
    ///
    /// - A `ur:crypto-pubkeys` (`PublicKeys`) with an SSH public key (exported
    ///   to single-line text),
    ///
    /// - An SSH `ur:signature` signature (exported to PEM).
    ///
    /// If not provided on the command line, the object will be read from
    /// stdin.
    ur_string: Option<String>,

    /// If present, the SSH private key will be encrypted with the given
    /// password.
    #[arg(long, default_value = "false")]
    encrypt: bool,

    /// The password to encrypt an SSH private key.
    ///
    /// If the `--encrypt` switch is present and this option is not provided,
    /// the password will be read interactively from the terminal if possible.
    #[arg(long)]
    password: Option<String>,

    #[arg(long, requires = "encrypt", help = ASKPASS_HELP, long_help = ASKPASS_LONG_HELP)]
    askpass: bool,
}

impl crate::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let object = read_argument(self.ur_string.as_deref())?;
        if let Ok(signing_private_key) =
            SigningPrivateKey::from_ur_string(&object)
        {
            if let Some(ssh_private_key) = signing_private_key.to_ssh() {
                if self.encrypt {
                    let mut rng = bc_rand::SecureRandomNumberGenerator;
                    let password = read_password(
                        "Key encryption password: ",
                        self.password.as_deref(),
                        self.askpass,
                    )?;
                    let openssh = ssh_private_key
                        .encrypt(&mut rng, password)?
                        .to_openssh(LineEnding::LF)?;
                    Ok(openssh.trim().to_string())
                } else {
                    Ok(ssh_private_key
                        .to_openssh(LineEnding::LF)?
                        .trim()
                        .to_string())
                }
            } else {
                bail!("UR is not an SSH private key.");
            }
        } else if let Ok(signing_public_key) =
            SigningPublicKey::from_ur_string(&object)
        {
            if let Some(ssh_public_key) = signing_public_key.to_ssh() {
                Ok(ssh_public_key.to_openssh()?.to_string())
            } else {
                bail!("UR is not an SSH public key.");
            }
        } else if let Ok(public_keys) = PublicKeys::from_ur_string(&object) {
            if let Some(ssh_public_key) =
                public_keys.signing_public_key().to_ssh()
            {
                Ok(ssh_public_key.to_openssh()?.to_string())
            } else {
                bail!("UR is not a PublicKeys with an SSH public key.");
            }
        } else if let Ok(signature) = Signature::from_ur_string(&object) {
            if let Some(ssh_signature) = signature.to_ssh() {
                Ok(ssh_signature.to_pem(LineEnding::LF)?.trim().to_string())
            } else {
                bail!("UR is not an SSH signature.");
            }
        } else {
            bail!(
                "Invalid object for export. Supported types are SSH `ur:signing-private-key`, SSH `ur:signing-public-key`, SSH `ur:crypto-pubkeys`, and `ur:signature`."
            );
        }
    }
}
