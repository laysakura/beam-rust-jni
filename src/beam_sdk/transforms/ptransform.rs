mod ptransform_id;

pub use ptransform_id::PTransformId;

use crate::beam_sdk::{
    error::BoxError,
    values::{PCollection, PCollectionValue},
};

pub trait PTransform {
    type InV: PCollectionValue;
    type OutV: PCollectionValue;

    fn id(&self) -> &PTransformId;

    fn apply(
        &self,
        in_pcollection: &PCollection<Self::InV>,
    ) -> Result<PCollection<Self::OutV>, BoxError>;
}
