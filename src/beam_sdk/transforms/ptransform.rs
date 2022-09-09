mod ptransform_id;

pub use ptransform_id::PTransformId;

use crate::beam_sdk::values::{PCollection, PCollectionValue};

pub trait PTransform {
    type InV: PCollectionValue;
    type OutV: PCollectionValue;

    fn id(&self) -> &PTransformId;

    fn apply(&self, in_pcollection: &PCollection<Self::InV>) -> PCollection<Self::OutV>;
}
