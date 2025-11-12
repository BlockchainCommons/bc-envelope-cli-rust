use anyhow::{Result, bail};
use clap::Args;

use crate::DataType;

pub trait SubjectArgsLike {
    fn subject_type(&self) -> DataType;
    fn subject_value(&self) -> Option<&str>;
    fn ur_tag(&self) -> Option<u64>;

    fn read_subject_value(&self) -> Result<String> {
        let mut string = String::new();
        if self.subject_value().is_none() {
            std::io::stdin().read_line(&mut string)?;
        } else {
            string = self.subject_value().as_ref().unwrap().to_string();
        }
        if string.is_empty() {
            bail!("No value provided");
        }
        Ok(string.trim().to_string())
    }
}

#[derive(Debug, Args)]
#[group(skip)]
pub struct SubjectArgs {
    /// Subject type.
    #[arg(name = "TYPE")]
    subject_type: DataType,
    /// Subject value.
    #[arg(name = "VALUE")]
    subject_value: Option<String>,
    /// The integer tag for an enclosed UR.
    #[arg(long)]
    ur_tag: Option<u64>,
}

impl SubjectArgsLike for SubjectArgs {
    fn subject_type(&self) -> DataType { self.subject_type }

    fn subject_value(&self) -> Option<&str> { self.subject_value.as_deref() }

    fn ur_tag(&self) -> Option<u64> { self.ur_tag }
}
