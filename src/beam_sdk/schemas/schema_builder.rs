use crate::beam_sdk::schemas::{
    schema_field::SchemaField, schema_field_type::SchemaFieldType, Schema,
};

#[derive(Debug)]
pub struct SchemaBuilder {
    fields: Vec<SchemaField>,
}

impl SchemaBuilder {
    pub fn new() -> Self {
        Self { fields: vec![] }
    }

    pub fn add<S>(self, field_name: S, field_type: SchemaFieldType) -> Self
    where
        S: ToString,
    {
        let field = SchemaField::new(field_name.to_string(), field_type);

        let mut fields = self.fields;
        fields.push(field);

        Self { fields }
    }

    pub fn build(self) -> Schema {
        Schema::new(self.fields)
    }
}
