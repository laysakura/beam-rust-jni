use crate::beam_sdk::{
    transforms::{ptransform::PTransformId, ReadTransform},
    values::Row,
};

#[derive(Debug)]
pub struct Create {
    id: PTransformId,
}

impl Create {
    pub fn from_row(row: Row) -> Self {
        Self {
            id: PTransformId::from("TODO unique id"),
        }
    }
}

impl ReadTransform for Create {}
