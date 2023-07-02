mod flags;
mod r#type;

pub use flags::Flags;
pub use r#type::FrameType;

#[derive(Debug, Clone, Copy, recode::Recode)]
pub struct FrameHeader {
    stream_id: u32,
    type_flags: u16,
}

impl FrameHeader {
    pub const fn stream_id(&self) -> u32 {
        self.stream_id
    }

    pub const fn frame_type(&self) -> FrameType {
        FrameType::from_base_type((self.type_flags >> 10) as u8)
    }

    pub const fn flags(&self) -> Flags {
        Flags::from_bits_truncate(self.type_flags % 0x03FF)
    }
}
