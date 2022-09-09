use crate::beam_sdk::{
    error::BoxError,
    transforms::{ptransform::PTransformId, PTransform},
    values::{PCollection, PCollectionId, Row},
};

#[derive(Debug)]
pub struct Select {
    id: PTransformId,
    /// TODO special types supporting `a.*`, etc.
    field_names: Vec<String>,
}

impl Select {
    pub fn field_names(field_names: &[&str]) -> Self {
        Self {
            id: PTransformId::from("TODO unique name"),
            field_names: field_names.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl PTransform for Select {
    type InV = Row; // TODO more specific schema types like PersonPojo
    type OutV = Row;

    fn id(&self) -> &super::ptransform::PTransformId {
        &self.id
    }

    fn apply(
        &self,
        in_pcollection: &PCollection<Self::InV>,
    ) -> Result<PCollection<Self::OutV>, BoxError> {
        let in_schema = in_pcollection.schema().expect("InV (Row) must have schema");
        let out_schema = in_schema.select(&self.field_names).map_err(Box::new)?;
        let out_pcollection =
            PCollection::new_row(PCollectionId::from("TODO unique id"), out_schema);
        Ok(out_pcollection)
    }
}
