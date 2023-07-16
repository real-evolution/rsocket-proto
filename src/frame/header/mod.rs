mod flags;
mod stream_id;
mod r#type;

pub use flags::Flags;
pub use r#type::FrameType;
pub use stream_id::StreamId;

/// A type to represent an rsocket frame header.
#[derive(Debug, Clone, Copy, recode::Recode)]
#[recode(error = "crate::Error")]
pub struct FrameHeader {
    stream_id: StreamId,
    type_flags: u16,
}

impl FrameHeader {
    /// The size of frame header in bytes.
    pub const SIZE: usize = 6;

    /// Creates a new frame header.
    ///
    /// # Parameters
    /// - `stream_id`: A stream identifier.
    /// - `frame_type`: A frame type.
    /// - `flags`: Frame flags.
    ///
    /// # Returns
    /// A new frame header.
    pub(crate) const fn new(
        stream_id: StreamId,
        frame_type: FrameType,
        flags: Flags,
    ) -> Self {
        let frame_type = (frame_type.to_base_type() as u16) << 10;

        Self {
            stream_id,
            type_flags: frame_type | flags.bits(),
        }
    }

    /// Gets frame stream identifier.
    pub const fn stream_id(&self) -> StreamId {
        self.stream_id
    }

    /// Gets frame type.
    pub const fn frame_type(&self) -> FrameType {
        FrameType::from_base_type((self.type_flags >> 10) as u8)
    }

    /// Gets frame flags.
    pub const fn flags(&self) -> Flags {
        Flags::from_bits_truncate(self.type_flags % 0x03FF)
    }

    /// A shorthand for `self.flags().contains(flag)`.
    #[inline]
    pub const fn has(&self, flag: Flags) -> bool {
        self.flags().contains(flag)
    }
}
