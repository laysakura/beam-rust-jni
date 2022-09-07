use crate::beam_sdk::{schemas::Schema, transforms::PTransform};

#[derive(Eq, PartialEq, Debug)]
pub struct PCollection {
    name: String,
}
impl PCollection {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }

    pub fn apply<P>(&self, transform: P) -> Self
    where
        P: PTransform,
    {
        todo!()
    }

    pub fn schema(&self) -> Option<&Schema> {
        todo!()
    }
}
