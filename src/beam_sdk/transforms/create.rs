use crate::beam_sdk::{
    schemas::Schema,
    transforms::{ptransform::PTransformId, ReadTransform},
    values::{PCollection, PCollectionId, PCollectionValue, Row},
};

#[derive(Debug)]
pub enum Create<V>
where
    V: PCollectionValue,
{
    Row {
        id: PTransformId,
        row: Row,
    },
    Value {
        id: PTransformId,
        value: V, // I guess Create must pass the value from SDK to runner but does Beam .proto do this?
    },
}

impl<V> Create<V>
where
    V: PCollectionValue,
{
    pub fn new_row(row: Row) -> Self {
        Self::Row {
            id: PTransformId::from("TODO unique id"),
            row,
        }
    }

    pub fn new_value(value: V) -> Self {
        Self::Value {
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
        let id = PCollectionId::from("TODO unique id");
        match self {
            Create::Row { row, .. } => PCollection::new_row(id, row.schema().clone()),
            Create::Value { .. } => PCollection::<Self::OutV>::new_value(id),
        }
    }
}
