use crate::beam_sdk::options::PipelineOptions;
use beam_proto_rs::v1::beam_runner_api::Pipeline as PipelineProto;

pub struct Pipeline;

impl Pipeline {
    pub fn new(options: PipelineOptions) -> Self {
        todo!()
    }
}

impl From<Pipeline> for PipelineProto {
    fn from(_: Pipeline) -> Self {
        todo!()
    }
}
