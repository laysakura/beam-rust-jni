mod pcollection_id;

pub use pcollection_id::PCollectionId;

use crate::beam_sdk::{schemas::Schema, transforms::PTransform};

#[derive(Eq, PartialEq, Debug)]
pub struct PCollection {
    id: PCollectionId,
}
impl PCollection {
    pub(crate) fn new(id: PCollectionId) -> Self {
        Self { id }
    }

    pub fn apply<P>(&self, transform: P) -> Self
    where
        P: PTransform,
    {
        Self::new(PCollectionId::from("TODO unique id"))
    }

    pub fn schema(&self) -> Option<&Schema> {
        None
    }
}
