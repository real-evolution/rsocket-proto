mod buffer;
mod header;
mod value_types;
mod variant;

use recode::bytes::Bytes;
pub use buffer::*;
pub use header::*;
pub use value_types::*;
pub use variant::*;

#[derive(Debug)]
pub struct Frame {
    header: FrameHeader,
    variant: FrameVariant,
}

impl Frame {
    #[inline]
    pub fn header(&self) -> &FrameHeader {
        &self.header
    }

    #[inline]
    pub fn variant(&self) -> &FrameVariant {
        &self.variant
    }
}
