use thiserror::Error;

/// A type alias for [`Result<T, [`Error>`]`].
pub type Result<T> = std::result::Result<T, Error>;

/// A type to represent all possible errors that can occur when using this
/// library.
#[derive(Debug, Error)]
pub enum Error {
    #[error("encoding/decoding error: {0}")]
    Codec(#[from] recode::Error),

    #[error("protocol violation: {0}")]
    ProtocolViolation(&'static str),
}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
