mod ptransform_id;

pub use ptransform_id::PTransformId;

pub type BoxedPTransform = Box<dyn PTransform>;

pub trait PTransform {
    fn id(&self) -> &PTransformId;
}
