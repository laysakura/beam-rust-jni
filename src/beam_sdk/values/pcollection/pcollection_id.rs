#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct PCollectionId(String);
impl<S> From<S> for PCollectionId
where
    S: ToString,
{
    fn from(s: S) -> Self {
        Self(s.to_string())
    }
}
