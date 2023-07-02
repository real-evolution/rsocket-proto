
use recode::bytes::Bytes;

#[derive(Debug)]
pub struct StreamFrame {
    pub stream_id: u32,
    pub frame: FrameVariant,
    pub raw: Bytes,
}

#[derive(Debug)]
pub enum FrameVariant {}
