use std::rc::Rc;
use bc_envelope::prelude::*;
use clap::Args;

pub trait EnvelopeArgsLike {
    fn envelope(&self) -> Option<&str>;

    fn get_envelope(&self) -> anyhow::Result<Rc<Envelope>> {
        let mut ur_string = String::new();
        if self.envelope().is_none() {
            std::io::stdin().read_line(&mut ur_string)?;
        } else {
            ur_string = self.envelope().as_ref().unwrap().to_string();
        }
        if ur_string.is_empty() {
            anyhow::bail!("No envelope provided");
        }
        let a = Rc::new(Envelope::from_ur_string(ur_string.trim())?);
        Ok(a)
    }
}

#[derive(Debug, Args)]
#[group(skip)]
pub struct EnvelopeArgs {
    /// The envelope to process. If the envelope is not supplied on the command line, it is read from stdin.
    envelope: Option<String>,
}

impl EnvelopeArgsLike for EnvelopeArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope.as_deref()
    }
}
