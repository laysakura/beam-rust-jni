mod pipeline_translation;

use crate::beam_sdk::{
    options::PipelineOptions,
    transforms::ReadTransform,
    values::{PCollection, PCollectionValue},
};

pub struct Pipeline {
    options: PipelineOptions,

    read_ptransforms: Vec<Box<dyn ReadTransform<OutV = dyn PCollectionValue>>>,
}

impl Pipeline {
    pub fn new(options: PipelineOptions) -> Self {
        Self {
            options,
            read_ptransforms: Vec::new(),
        }
    }

    pub fn apply<R>(&self, root: R) -> PCollection<R::OutV>
    where
        R: ReadTransform,
    {
        root.out_pcollection()
    }
}
