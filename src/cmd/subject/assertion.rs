use bc_ur::UREncodable;
use clap::Args;
use crate::data_types::{DataType, parse_data_type_to_envelope};
use bc_envelope::preamble::*;

/// Create an envelope with the given assertion (predicate and object).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Predicate type.
    #[arg(name = "PRED_TYPE")]
    predicate_type: DataType,
    /// Predicate value.
    #[arg(name = "PRED_VALUE")]
    predicate_value: String,
    /// Object type.
    #[arg(name = "OBJ_TYPE")]
    object_type: DataType,
    /// Object value.
    #[arg(name = "OBJ_VALUE")]
    object_value: String,
    /// The integer tag for the predicate provided as an enclosed UR.
    #[arg(long)]
    pred_tag: Option<u64>,
    /// The integer tag for the object provided as an enclosed UR.
    #[arg(long)]
    obj_tag: Option<u64>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let predicate = parse_data_type_to_envelope(self.predicate_type, &self.predicate_value, self.pred_tag)?;
        let object = parse_data_type_to_envelope(self.object_type, &self.object_value, self.obj_tag)?;
        let envelope = Envelope::new_assertion(predicate, object);
        Ok(envelope.ur_string())
    }
}
