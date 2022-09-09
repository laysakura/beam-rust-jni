use crate::beam_sdk::{
    schemas::Schema,
    transforms::{ptransform::PTransformId, ReadTransform},
    values::{PCollection, PCollectionId, PCollectionValue, Row},
};

#[derive(Debug)]
pub struct Create<V>
where
    V: PCollectionValue,
{
    id: PTransformId,
    value: V, // I guess Create must pass the value from SDK to runner but does Beam .proto do this?
}

impl<V> Create<V>
where
    V: PCollectionValue,
{
    pub fn from_row(value: V) -> Self {
        Self {
            id: PTransformId::from("TODO unique id"),
            value,
        }
    }
}

impl<V> ReadTransform for Create<V>
where
    V: PCollectionValue,
{
    type OutV = V;

    fn out_pcollection(&self) -> PCollection<Self::OutV> {
        PCollection::<Self::OutV>::new_value(PCollectionId::from("TODO unique id"))
    }
}
