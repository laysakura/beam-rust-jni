use crate::beam_sdk::{schemas::Schema, values::PCollectionValue};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Row {
    schema: Schema,
}

impl Row {
    pub(crate) fn new(schema: Schema) -> Self {
        Self { schema }
    }

    pub fn as_schema(&self) -> &Schema {
        &self.schema
    }
}

impl PCollectionValue for Row {}
