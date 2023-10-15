use std::{rc::Rc, collections::HashSet};
use bc_envelope::prelude::*;

pub fn read_envelope(envelope: Option<&str>) -> Result<Rc<Envelope>, anyhow::Error> {
    let mut ur_string = String::new();
    if envelope.is_none() {
        std::io::stdin().read_line(&mut ur_string)?;
    } else {
        ur_string = envelope.as_ref().unwrap().to_string();
    }
    if ur_string.is_empty() {
        anyhow::bail!("No envelope provided");
    }
    Ok(Rc::new(Envelope::from_ur_string(ur_string.trim())?))
}

pub fn parse_digest(target: &str) -> Result<Digest, anyhow::Error> {
    let ur = UR::from_ur_string(target)?;
    let digest = match ur.ur_type() {
        "digest" => {
            Digest::from_ur(&ur)?
        },
        "envelope" => {
            Envelope::from_ur(&ur)?.digest().into_owned()
        }
        _ => {
            anyhow::bail!("Invalid digest type: {}", ur.ur_type());
        }
    };
    Ok(digest)
}

pub fn parse_digests(target: &str) -> Result<HashSet<Digest>, anyhow::Error> {
    let target = target.trim();
    if target.is_empty() {
        Ok(HashSet::new())
    } else {
        target.split(' ').map(parse_digest).collect::<anyhow::Result<HashSet<Digest>>>()
    }
}
