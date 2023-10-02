use bc_envelope::prelude::*;
use clap::Args;

use crate::{pred_obj_args::{PredObjArgs, PredObjArgsLike}, data_types::DataType, envelope_args::{EnvelopeArgs, EnvelopeArgsLike}};

/// Add an assertion with the given predicate and object to the given envelope.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    assertion_args: PredObjArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl PredObjArgsLike for CommandArgs {
    fn pred_type(&self) -> DataType {
        self.assertion_args.pred_type()
    }
    fn pred_value(&self) -> &str {
        self.assertion_args.pred_value()
    }
    fn obj_type(&self) -> DataType {
        self.assertion_args.obj_type()
    }
    fn obj_value(&self) -> &str {
        self.assertion_args.obj_value()
    }
    fn pred_tag(&self) -> Option<u64> {
        self.assertion_args.pred_tag()
    }
    fn obj_tag(&self) -> Option<u64> {
        self.assertion_args.obj_tag()
    }
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let envelope = self.read_envelope()?;
        let assertion = self.assertion_envelope()?;
        Ok(envelope.add_assertion_envelope(assertion)?.ur_string())
    }
}
