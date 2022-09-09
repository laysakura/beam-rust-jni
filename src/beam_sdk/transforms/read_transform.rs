use crate::beam_sdk::values::{PCollection, PCollectionValue};

pub trait ReadTransform {
    type OutV: PCollectionValue;

    fn out_pcollection(&self) -> PCollection<Self::OutV>;
}
