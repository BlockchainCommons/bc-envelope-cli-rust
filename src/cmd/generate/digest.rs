use std::io::Read;

use anyhow::Result;
use bc_envelope::prelude::*;
use clap::Args;

/// Generate a digest from the input data.
///
/// If the `data` argument is given on the command line, it is taken as a UTF-8
/// string. If it is omitted on the command line, then all available data is
/// read from stdin and treated as a binary blob.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The data to digest.
    data: Option<String>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let mut data = Vec::new();
        if let Some(ref d) = self.data {
            data.extend_from_slice(d.as_bytes());
        } else {
            std::io::stdin().read_to_end(&mut data)?;
        }
        let digest = Digest::from_image(&data);
        Ok(digest.ur_string())
    }
}
