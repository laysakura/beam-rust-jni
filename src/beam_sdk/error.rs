use anyhow::anyhow;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{}", .0)]
pub struct InvalidArgsError(anyhow::Error);

impl InvalidArgsError {
    pub fn new(arg: String) -> Self {
        Self(anyhow!("Invalid argument: {}", arg))
    }
}
