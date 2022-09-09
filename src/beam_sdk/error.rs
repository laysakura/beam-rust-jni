use std::error;

use anyhow::anyhow;
use thiserror::Error;

use crate::beam_sdk::schemas::{Schema, SchemaFieldType};

pub type BoxError = Box<dyn error::Error + Sync + Send + 'static>;

#[derive(Debug, Error)]
#[error("{}", .0)]
pub struct FieldTypeError(anyhow::Error);

impl FieldTypeError {
    pub(crate) fn new(
        field_name: &str,
        typ: SchemaFieldType,
        expected_type: SchemaFieldType,
    ) -> Self {
        Self(anyhow!(
            "Incompatible type for field {} (given {} ; expected {})",
            field_name,
            typ,
            expected_type
        ))
    }
}

#[derive(Debug, Error)]
#[error("{}", .0)]
pub struct FieldNameNotFound(anyhow::Error);

impl FieldNameNotFound {
    pub(crate) fn new(field_name: &str, schema: &Schema) -> Self {
        Self(anyhow!(
            "Field {} does not exist in schema: {}",
            field_name,
            schema
        ))
    }
}
