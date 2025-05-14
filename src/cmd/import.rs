use anyhow::{bail, Result};
use clap::Args;

use crate::utils::{read_argument, read_password};
use bc_components::{Signature, SigningPrivateKey, SigningPublicKey};
use bc_envelope::prelude::*;
use ssh_key::{PrivateKey as SSHPrivateKey, PublicKey as SSHPublicKey, SshSig as SSHSignature};

use super::{ASKPASS_HELP, ASKPASS_LONG_HELP};

/// Import the given object to UR form.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The object to be imported into UR form.
    /// 
    /// May be an Open SSH private key (PEM), public key (single-line), or signature (PEM).
    ///
    /// If not provided on the command line, the object will be read from stdin.
    object: Option<String>,

    /// The password to decrypt the SSH private key.
    ///
    /// If the SSH private key is encrypted, this is required. If not provided
    /// on the command line, the password will be read interactively from the
    /// terminal if possible.
    #[arg(long)]
    password: Option<String>,

    #[arg(long, help = ASKPASS_HELP, long_help = ASKPASS_LONG_HELP)]
    askpass: bool,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let object = read_argument(self.object.as_deref())?;
        let result = if let Ok(ssh_private_key) = SSHPrivateKey::from_openssh(&object) {
            if ssh_private_key.is_encrypted() {
                let password = read_password("Key decryption password: ", self.password.as_deref(), self.askpass)?;
                SigningPrivateKey::new_ssh(ssh_private_key.decrypt(password)?).ur_string()
            } else {
                SigningPrivateKey::new_ssh(ssh_private_key).ur_string()
            }
        } else if let Ok(ssh_public_key) = SSHPublicKey::from_openssh(&object) {
            SigningPublicKey::from_ssh(ssh_public_key).ur_string()
        } else if let Ok(ssh_signature) = SSHSignature::from_pem(&object) {
            Signature::from_ssh(ssh_signature).ur_string()
        } else {
            bail!("Invalid object for import. Supported types are SSH private key, public key, and signature.");
        };
        Ok(result)
    }
}
