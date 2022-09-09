mod pcollection_id;

use std::marker::PhantomData;

pub use pcollection_id::PCollectionId;

use crate::beam_sdk::{schemas::Schema, transforms::PTransform, values::PCollectionValue};

pub struct PCollection<V>(PCollectionInner<V>)
where
    V: PCollectionValue;

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum PCollectionInner<V>
where
    V: PCollectionValue,
{
    Row {
        schema: Schema,
        common: PCollectionCommon,
    },
    Value {
        _value: PhantomData<V>,
        common: PCollectionCommon,
    },
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct PCollectionCommon {
    id: PCollectionId,
}
impl<V> PCollection<V>
where
    V: PCollectionValue,
{
    pub(crate) fn new_row(id: PCollectionId, schema: Schema) -> Self {
        let common = PCollectionCommon { id };
        let inner = PCollectionInner::Row { common, schema };
        Self(inner)
    }

    pub(crate) fn new_value(id: PCollectionId) -> Self {
        let common = PCollectionCommon { id };
        let inner = PCollectionInner::Value {
            common,
            _value: PhantomData::<V>::default(),
        };
        Self(inner)
    }

    pub fn apply<P, OV>(&self, transform: P) -> PCollection<OV>
    where
        P: PTransform<InV = V, OutV = OV>,
        OV: PCollectionValue,
    {
        transform.apply(self)
    }

    pub fn schema(&self) -> Option<&Schema> {
        match &self.0 {
            PCollectionInner::Row { schema, .. } => Some(schema),
            _ => None,
        }
    }
}
