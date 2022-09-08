mod pipeline_translation;

use std::{cell::RefCell, rc::Rc};

use crate::beam_sdk::{
    options::PipelineOptions,
    transforms::{BoxedPTransform, ReadTransform},
    values::{PCollection, PCollectionId},
};
use petgraph::{
    graph::{DiGraph, NodeIndex},
    visit::IntoNodeReferences,
};

type Graph = Rc<RefCell<DiGraph<BoxedPTransform, PCollection>>>;

pub struct Pipeline {
    options: PipelineOptions,
    graph: Graph,
}

impl Pipeline {
    pub fn new(options: PipelineOptions) -> Self {
        Self {
            options,
            graph: Graph::default(),
        }
    }

    pub fn apply<R>(&self, root: R) -> PCollection
    where
        R: ReadTransform,
    {
        PCollection::new(PCollectionId::from("TODO unique id"))
    }
}
