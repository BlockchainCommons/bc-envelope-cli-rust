use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

use crate::data_types::{DataType, parse_data_type_to_envelope};

pub trait PredObjArgsLike {
    fn pred_type(&self) -> DataType;
    fn pred_value(&self) -> &str;
    fn obj_type(&self) -> DataType;
    fn obj_value(&self) -> &str;
    fn pred_tag(&self) -> Option<u64>;
    fn obj_tag(&self) -> Option<u64>;

    fn assertion_envelope(&self) -> Result<Envelope> {
        let predicate = parse_data_type_to_envelope(
            self.pred_type(),
            Some(self.pred_value()),
            self.pred_tag(),
        )?;
        let object = parse_data_type_to_envelope(
            self.obj_type(),
            Some(self.obj_value()),
            self.obj_tag(),
        )?;
        Ok(Envelope::new_assertion(predicate, object))
    }
}

/// Create an envelope with the given assertion (predicate and object).
#[derive(Debug, Args)]
#[group(skip)]
pub struct PredObjArgs {
    /// Predicate type.
    pred_type: DataType,
    /// Predicate value.
    pred_value: String,
    /// Object type.
    obj_type: DataType,
    /// Object value.
    obj_value: String,
    /// The integer tag for the predicate provided as an enclosed UR.
    #[arg(long)]
    pred_tag: Option<u64>,
    /// The integer tag for the object provided as an enclosed UR.
    #[arg(long)]
    obj_tag: Option<u64>,
}

impl PredObjArgsLike for PredObjArgs {
    fn pred_type(&self) -> DataType { self.pred_type }
    fn pred_value(&self) -> &str { &self.pred_value }
    fn obj_type(&self) -> DataType { self.obj_type }
    fn obj_value(&self) -> &str { &self.obj_value }
    fn pred_tag(&self) -> Option<u64> { self.pred_tag }
    fn obj_tag(&self) -> Option<u64> { self.obj_tag }
}
