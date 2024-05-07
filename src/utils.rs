use std::collections::HashSet;
use anyhow::{bail, Result};
use bc_envelope::prelude::*;

pub fn read_envelope(envelope: Option<&str>) -> Result<Envelope> {
    let mut ur_string = String::new();
    if envelope.is_none() {
        std::io::stdin().read_line(&mut ur_string)?;
    } else {
        ur_string = envelope.as_ref().unwrap().to_string();
    }
    if ur_string.is_empty() {
        bail!("No envelope provided");
    }
    Envelope::from_ur_string(ur_string.trim())
}

pub fn parse_digest(target: &str) -> Result<Digest> {
    let ur = UR::from_ur_string(target)?;
    let digest = match ur.ur_type_str() {
        "digest" => {
            Digest::from_ur(&ur)?
        },
        "envelope" => {
            Envelope::from_ur(&ur)?.digest().into_owned()
        }
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
        target.split(' ').map(parse_digest).collect::<Result<HashSet<Digest>>>()
    }
}
