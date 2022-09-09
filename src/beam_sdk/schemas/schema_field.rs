use crate::beam_sdk::schemas::SchemaFieldType;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct SchemaField {
    name: String,
    typ: SchemaFieldType,
}

impl SchemaField {
    pub(crate) fn new(field_name: String, field_type: SchemaFieldType) -> Self {
        Self {
            name: field_name,
            typ: field_type,
        }
    }

    pub(crate) fn hit(&self, field_name: &str) -> bool {
        &self.name == field_name
    }
}
