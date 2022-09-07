use std::env;

use crate::beam_sdk::error::InvalidArgsError;

pub struct PipelineOptions;

impl PipelineOptions {
    pub fn from_args(args: env::Args) -> Result<Self, InvalidArgsError> {
        todo!()
    }
}
