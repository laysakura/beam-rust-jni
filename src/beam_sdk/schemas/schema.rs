use crate::beam_sdk::schemas::schema_field::SchemaField;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Schema {
    fields: Vec<SchemaField>,
}

impl Schema {
    pub(crate) fn new(fields: Vec<SchemaField>) -> Self {
        Self { fields }
    }
}
