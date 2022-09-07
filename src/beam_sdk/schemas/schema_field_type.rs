use std::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SchemaFieldType {
    Int16,
    Int32,
    String,
}

impl Display for SchemaFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SchemaFieldType::Int16 => "INT16",
            SchemaFieldType::Int32 => "INT32",
            SchemaFieldType::String => "STRING",
        };
        write!(f, "{}", s)
    }
}
