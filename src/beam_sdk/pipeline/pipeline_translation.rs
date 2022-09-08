use beam_proto_rs::v1::beam_runner_api::Pipeline as PipelineProto;

use crate::beam_sdk::Pipeline;

impl From<Pipeline> for PipelineProto {
    fn from(_: Pipeline) -> Self {
        todo!()
    }
}
