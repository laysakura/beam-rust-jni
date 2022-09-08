use std::env;

use crate::beam_sdk::{error::InvalidArgsError, runners::PipelineRunnerKind};

#[derive(Eq, PartialEq, Debug)]
pub struct PipelineOptions {
    /// The pipeline runner that will be used to execute the pipeline.
    runner: PipelineRunnerKind,
}

impl PipelineOptions {
    pub fn from_args(args: env::Args) -> Result<Self, InvalidArgsError> {
        todo!()
    }
}
