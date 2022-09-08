#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct PTransformId(String);
impl<S> From<S> for PTransformId
where
    S: ToString,
{
    fn from(s: S) -> Self {
        Self(s.to_string())
    }
}
