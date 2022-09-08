use crate::beam_sdk::schemas::{schema_field_type::SchemaFieldType, Schema};

#[derive(Debug)]
pub struct SchemaBuilder {}

impl SchemaBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add<S>(self, field_name: S, field_type: SchemaFieldType) -> Self
    where
        S: ToString,
    {
        self
    }

    pub fn build(self) -> Schema {
        Schema {}
    }
}
