use nom::{error::Error as NomError, Err as NomErr};
use thiserror::Error;

use crate::frame::Flags;

/// A type alias for [`Result<T, [`RSocketError>`]`].
pub type RSocketResult<T> = Result<T, RSocketError>;

/// A type to represent all possible errors that can occur when using this
/// library.
#[derive(Debug, Error)]
pub enum RSocketError {
    #[error("parsing error (0x{:08X}): {}", nom::error::error_to_u32(.0), .0.description())]
    Parsing(nom::error::ErrorKind),

    #[error("invalid buffer size: {0}")]
    BufferLength(&'static str),

    #[error("invalid stream identifier: expected 0x{expected:08X}, got 0x{actual:08X}")]
    UnexpectedStreamId { expected: u32, actual: u32 },

    #[error("invalid flags value: 0x{flags:04X}, mask = 0x{mask:04X}")]
    UnexpectedFlags { flags: Flags, mask: Flags },

    #[error("unexpected flag value: {flag:?}, expected = {expected_value}")]
    UnexpectedFlagValue { flag: Flags, expected_value: bool },
}

impl<I> From<NomErr<NomError<I>>> for RSocketError {
    fn from(value: NomErr<NomError<I>>) -> Self {
        match value {
            | nom::Err::Incomplete(_) => {
                Self::BufferLength("input buffer too short")
            }
            | NomErr::Error(e) | NomErr::Failure(e) => Self::Parsing(e.code),
        }
    }
}
