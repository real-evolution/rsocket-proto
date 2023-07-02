mod buffer;

use recode::bytes::Bytes;
pub use buffer::*;

#[derive(Debug)]
pub struct StreamFrame {
    pub stream_id: u32,
    pub frame: FrameVariant,
    pub raw: Bytes,
}

#[derive(Debug)]
pub enum FrameVariant {}
