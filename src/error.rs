use thiserror::Error;

use crate::frame::Flags;

/// A type alias for [`Result<T, [`RSocketError>`]`].
pub type RSocketResult<T> = Result<T, RSocketError>;

/// A type to represent all possible errors that can occur when using this
/// library.
#[derive(Debug, Error)]
pub enum RSocketError {
    #[error("invalid stream identifier: expected 0x{expected:08X}, got 0x{actual:08X}")]
    InvalidStreamId { expected: u32, actual: u32 },

    #[error("invalid flags value: 0x{flags:04X}, mask = 0x{mask:04X}")]
    InvalidFlags { flags: Flags, mask: Flags },

    #[error("unexpected flag value: {flag:?}, expected = {expected_value}")]
    UnexpectedFlagValue { flag: Flags, expected_value: bool },
}
