use crate::beam_sdk::transforms::PTransform;

#[derive(Debug)]
pub struct Select;

impl Select {
    pub fn field_names(field_names: &[&str]) -> Self {
        todo!()
    }
}

impl PTransform for Select {}
