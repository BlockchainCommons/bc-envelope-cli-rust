use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::{
    data_types::DataType,
    pred_obj_args::{PredObjArgs, PredObjArgsLike},
};

/// Create a bare assertion with the given predicate and object.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    pred_obj_args: PredObjArgs,

    #[arg(short, long, default_value = "false")]
    salted: bool,
}

impl PredObjArgsLike for CommandArgs {
    fn pred_type(&self) -> DataType { self.pred_obj_args.pred_type() }
    fn pred_value(&self) -> &str { self.pred_obj_args.pred_value() }
    fn obj_type(&self) -> DataType { self.pred_obj_args.obj_type() }
    fn obj_value(&self) -> &str { self.pred_obj_args.obj_value() }
    fn pred_tag(&self) -> Option<u64> { self.pred_obj_args.pred_tag() }
    fn obj_tag(&self) -> Option<u64> { self.pred_obj_args.obj_tag() }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let mut result = self.pred_obj_args.assertion_envelope()?;
        if self.salted {
            result = result.add_salt();
        }
        Ok(result.ur_string())
    }
}
