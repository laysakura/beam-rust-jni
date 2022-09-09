use std::fmt::Display;

use crate::beam_sdk::{error::FieldNameNotFound, schemas::schema_field::SchemaField};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Schema {
    fields: Vec<SchemaField>,
}

impl Schema {
    pub(crate) fn new(fields: Vec<SchemaField>) -> Self {
        Self { fields }
    }

    pub(crate) fn select(&self, field_names: &[String]) -> Result<Self, FieldNameNotFound> {
        let new_fields = field_names
            .iter()
            .map(|field_name| {
                self.fields
                    .iter()
                    .find(|schema_field| schema_field.hit(field_name))
                    .cloned()
                    .ok_or_else(|| FieldNameNotFound::new(field_name, self))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { fields: new_fields })
    }
}

impl Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // TODO prettier 1 line
    }
}
