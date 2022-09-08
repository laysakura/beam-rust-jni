use crate::beam_sdk::runners::PipelineRunnerKind;

#[derive(Eq, PartialEq, Debug)]
pub struct PipelineOptions {
    /// The pipeline runner that will be used to execute the pipeline.
    runner: PipelineRunnerKind,
}

impl PipelineOptions {
    pub fn new(runner: PipelineRunnerKind) -> Self {
        Self { runner }
    }
}
