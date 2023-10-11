use std::rc::Rc;
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

pub fn read_optional_envelope(envelope: Option<&str>) -> Result<Option<Rc<Envelope>>, anyhow::Error> {
    if let Some(envelope) = envelope {
        Ok(Some(read_envelope(Some(envelope))?))
    } else {
        Ok(None)
    }
}
