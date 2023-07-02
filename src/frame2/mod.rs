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
pub struct StreamFrame {
    pub stream_id: u32,
    pub frame: FrameVariant,
    pub raw: Bytes,
}

