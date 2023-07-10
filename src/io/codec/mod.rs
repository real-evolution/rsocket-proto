mod composed;
mod decoder;
mod encoder;
mod fragmented;

pub use decoder::FrameDecoder;
pub use encoder::FrameEncoder;
pub use fragmented::{FragmentedFrameDecoder, FragmentedFrameEncoder};
pub use self::composed::ComposedCodec;

pub const MAX_FRAME_LEN: usize = 16 * 1024 * 1024;
