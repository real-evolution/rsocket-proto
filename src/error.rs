use thiserror::Error;

/// A type alias for [`Result<T, [`Error>`]`].
pub type Result<T> = std::result::Result<T, Error>;

/// A type to represent all possible errors that can occur when using this
/// library.
#[derive(Debug, Error)]
pub enum Error {
    #[error("encoding/decoding error: {0}")]
    Codec(#[from] recode::Error),

    #[error("invalid value: {0}")]
    InvalidValue(String),

    #[error("invalid frame type: {0}")]
    UnsupportedFrameType(crate::frame::FrameType),

    #[error("missing flag `{flag}' in `{frame_type}': {message}")]
    MissingFlag {
        flag: crate::frame::Flags,
        frame_type: crate::frame::FrameType,
        message: &'static str,
    },

    #[error("unexpected flag `{flag}' in `{frame_type}': {message}")]
    UnexpectedFlag {
        flag: crate::frame::Flags,
        frame_type: crate::frame::FrameType,
        message: &'static str,
    },

    #[error(
        "unexpected end of a `{}` frame in stream #{:?}: {}",
        frame_type,
        stream_id,
        message
    )]
    UnexpectedEndOfFrame {
        stream_id: crate::frame::StreamId,
        frame_type: crate::frame::FrameType,
        message: &'static str,
    },

    #[error("missing value of `{frame_type} :: {field}'")]
    MissingFieldValue {
        frame_type: crate::frame::FrameType,
        field: &'static str,
    },
}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
