use crate::beam_sdk::{
    schemas::Schema,
    transforms::{ptransform::PTransformId, PTransform},
    values::{PCollection, Row},
};

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
    type InV = Row; // TODO more specific schema types like PersonPojo
    type OutV = Row;

    fn id(&self) -> &super::ptransform::PTransformId {
        &self.id
    }

    fn apply(&self, in_pcollection: &PCollection<Self::InV>) -> PCollection<Self::OutV> {
        let in_schema = in_pcollection.schema();

        todo!()
    }
}
