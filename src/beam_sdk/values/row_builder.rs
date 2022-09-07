use crate::beam_sdk::{error::FieldTypeError, schemas::Schema, values::row::Row};

#[derive(Debug)]
pub struct RowBuilder;

impl RowBuilder {
    pub fn new(schema: &Schema) -> Self {
        todo!()
    }

    pub fn add_value<S: AsRef<str>, T>(self, field_name: S, value: T) -> Self {
        todo!()
    }

    pub fn build(self) -> Result<Row, FieldTypeError> {
        todo!()
    }
}
