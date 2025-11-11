use std::{
    collections::HashSet,
    env,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
    str,
};

use anyhow::{Result, bail};
use bc_components::XID;
use bc_envelope::prelude::*;
use bc_xid::XIDDocument;

/// Reads a password either from the provided argument, via the system's askpass
/// tool when enabled, or interactively via rpassword.
///
/// # Arguments
///
/// * `prompt` - The prompt to show the user.
/// * `password` - An optional password string.
/// * `use_askpass` - Boolean flag to indicate if the system's askpass should be
///   used.
///
/// # Returns
///
/// A Result wrapping the password string.
/// Ask the user for a password, honoring explicit overrides, graphical helpers,
/// and finally falling back to a plain TTY prompt.
pub fn read_password(
    prompt: &str,
    password_override: Option<&str>,
    use_askpass: bool,
) -> anyhow::Result<String> {
    // 1. If the caller already supplied a password, trust it and return.
    if let Some(p) = password_override {
        return Ok(p.to_owned());
    }

    // 2. If the caller wants a GUI prompt, try to discover and invoke one.
    let password = if use_askpass {
        if let Some(cmd) = resolve_askpass() {
            let out = Command::new(cmd).arg(prompt).output()?;

            if out.status.success() {
                let pass = str::from_utf8(&out.stdout)
                    .map_err(|e| {
                        anyhow::anyhow!("askpass produced invalid UTF‑8: {e}")
                    })?
                    .trim_end_matches(&['\n', '\r'][..])
                    .to_owned();
                Some(pass)
            } else {
                // A non‑zero exit from askpass is treated as a soft failure;
                // we fall through to the TTY prompt instead of aborting.
                eprintln!("askpass exited with {}", out.status);
                None
            }
        } else {
            None
        }
    } else {
        None
    }
    .unwrap_or_else(|| {
        // 3. Last resort: prompt on the terminal.
        rpassword::prompt_password(prompt).unwrap_or_default()
    });

    // 4. If the password is empty, return an error.
    if password.is_empty() {
        bail!("Password cannot be empty");
    }
    Ok(password)
}

/// Locate a suitable askpass helper.
///
/// The search order is:
///   •  `$SSH_ASKPASS` or `$ASKPASS` if either is set.
///   •  Well‑known install locations on macOS and Linux.
///   •  The first `askpass`‑named binary found in `$PATH`.
fn resolve_askpass() -> Option<PathBuf> {
    // Explicit environment overrides take precedence.
    if let Ok(path) = env::var("SSH_ASKPASS").or_else(|_| env::var("ASKPASS")) {
        return Some(PathBuf::from(path));
    }

    // Common absolute paths used by package managers and system installs.
    const CANDIDATES: &[&str] = &[
        "/usr/libexec/ssh-askpass",
        "/usr/lib/ssh/ssh-askpass",
        "/usr/local/bin/ssh-askpass",
        "/opt/homebrew/bin/ssh-askpass",
    ];
    for cand in CANDIDATES {
        let p = Path::new(cand);
        if p.exists() && p.is_file() {
            return Some(p.to_path_buf());
        }
    }

    // Finally, fall back to whatever “askpass” the user might have on $PATH.
    which::which("askpass").ok()
}

pub fn read_argument(argument: Option<&str>) -> Result<String> {
    let string = if let Some(arg) = argument {
        arg.to_string()
    } else {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s)?;
        s
    };
    if string.is_empty() {
        bail!("No argument provided");
    }
    Ok(string)
}

pub fn read_envelope(envelope: Option<&str>) -> Result<Envelope> {
    let ur_string = if let Some(env) = envelope {
        env.to_string()
    } else {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s)?;
        s
    };
    if ur_string.is_empty() {
        bail!("No envelope provided");
    }
    // Just try to parse the envelope as a ur:envelope string first
    if let Ok(envelope) = Envelope::from_ur_string(ur_string.trim()) {
        Ok(envelope)
        // If that fails, try to parse the envelope as a ur:<any> string
    } else if let Ok(ur) = UR::from_ur_string(ur_string.trim()) {
        let cbor = ur.cbor();
        // Try to parse the CBOR into an envelope
        if let Ok(envelope) = Envelope::from_tagged_cbor(cbor) {
            Ok(envelope)
        } else if ur.ur_type_str() == "xid" {
            let xid = XID::from_untagged_cbor(ur.cbor())?;
            let doc = XIDDocument::from(xid);
            Ok(doc.into_envelope())
        } else {
            todo!();
        }
    } else {
        bail!("Invalid envelope");
    }
}

pub fn parse_digest(target: &str) -> Result<Digest> {
    let ur = UR::from_ur_string(target)?;
    let digest = match ur.ur_type_str() {
        "digest" => Digest::from_ur(&ur)?,
        "envelope" => Envelope::from_ur(&ur)?.digest().into_owned(),
        _ => {
            bail!("Invalid digest type: {}", ur.ur_type_str());
        }
    };
    Ok(digest)
}

pub fn parse_digests(target: &str) -> Result<HashSet<Digest>> {
    let target = target.trim();
    if target.is_empty() {
        Ok(HashSet::new())
    } else {
        target
            .split(' ')
            .map(parse_digest)
            .collect::<Result<HashSet<Digest>>>()
    }
}
