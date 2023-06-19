use thiserror::Error;

/// A type alias for [`Result<T, [`RSocketError>`]`].
pub type RSocketResult<T> = Result<T, RSocketError>;

/// A type to represent all possible errors that can occur when using this
/// library.
#[derive(Debug, Error)]
pub enum RSocketError {
    #[error("invalid flags value: 0x{0:02X}")]
    InvalidFlags(u16)
}
