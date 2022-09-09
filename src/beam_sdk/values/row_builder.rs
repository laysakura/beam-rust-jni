use crate::beam_sdk::{error::FieldTypeError, schemas::Schema, values::row::Row};

#[derive(Debug)]
pub struct RowBuilder {
    schema: Schema,
}

impl RowBuilder {
    pub fn new(schema: &Schema) -> Self {
        Self {
            schema: schema.clone(),
        }
    }

    pub fn add_value<S: AsRef<str>, T>(self, field_name: S, value: T) -> Self {
        self
    }

    pub fn build(self) -> Result<Row, FieldTypeError> {
        // TODO check with schema
        Ok(Row::new(self.schema))
    }
}
