use crate::beam_sdk::{transforms::ReadTransform, values::Row};

#[derive(Debug)]
pub struct Create;

impl Create {
    pub fn from_row(row: Row) -> Self {
        todo!()
    }
}

impl ReadTransform for Create {}
