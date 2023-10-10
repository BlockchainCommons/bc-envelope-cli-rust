use std::rc::Rc;
use bc_envelope::prelude::*;
use clap::Args;

use crate::utils::read_envelope;

pub trait EnvelopeArgsLike {
    fn envelope(&self) -> Option<&str>;

    fn read_envelope(&self) -> anyhow::Result<Rc<Envelope>> {
        read_envelope(self.envelope())
    }
}

#[derive(Debug, Args)]
#[group(skip)]
pub struct EnvelopeArgs {
    /// The envelope to process.
    ///
    /// If the envelope is not supplied on the command line, it is read from stdin.
    envelope: Option<String>,
}

impl EnvelopeArgsLike for EnvelopeArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope.as_deref()
    }
}
