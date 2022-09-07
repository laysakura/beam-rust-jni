use anyhow::anyhow;
use thiserror::Error;

use crate::beam_sdk::schemas::SchemaFieldType;

#[derive(Debug, Error)]
#[error("{}", .0)]
pub struct InvalidArgsError(anyhow::Error);

impl InvalidArgsError {
    pub(crate) fn new(arg: String) -> Self {
        Self(anyhow!("Invalid argument: {}", arg))
    }
}

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
