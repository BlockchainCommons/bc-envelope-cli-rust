use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{
    data_types::DataType,
    pred_obj_args::{PredObjArgs, PredObjArgsLike},
};

/// Create an envelope with the given assertion (predicate and object).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    assertion_args: PredObjArgs,
}

impl PredObjArgsLike for CommandArgs {
    fn pred_type(&self) -> DataType { self.assertion_args.pred_type() }
    fn pred_value(&self) -> &str { self.assertion_args.pred_value() }
    fn obj_type(&self) -> DataType { self.assertion_args.obj_type() }
    fn obj_value(&self) -> &str { self.assertion_args.obj_value() }
    fn pred_tag(&self) -> Option<u64> { self.assertion_args.pred_tag() }
    fn obj_tag(&self) -> Option<u64> { self.assertion_args.obj_tag() }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        Ok(self.assertion_envelope()?.ur_string())
    }
}
