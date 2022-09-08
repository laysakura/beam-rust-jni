use crate::beam_sdk::transforms::{ptransform::PTransformId, PTransform};

#[derive(Debug)]
pub struct Select {
    id: PTransformId,
}

impl Select {
    pub fn field_names(field_names: &[&str]) -> Self {
        Self {
            id: PTransformId::from("TODO unique name"),
        }
    }
}

impl PTransform for Select {
    fn id(&self) -> &super::ptransform::PTransformId {
        &self.id
    }
}
