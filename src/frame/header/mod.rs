mod flags;
mod r#type;

pub use flags::Flags;
pub use r#type::FrameType;

/// A type to represent an rsocket frame header.
#[derive(Debug, Clone, Copy, recode::Recode)]
#[recode(error = "crate::Error")]
pub struct FrameHeader {
    stream_id: u32,
    type_flags: u16,
}

impl FrameHeader {
    /// Gets frame stream identifier.
    pub const fn stream_id(&self) -> u32 {
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
}
