mod composed;
mod decoder;
mod encoder;
mod fragmented;

pub use self::composed::ComposedCodec;
pub use self::decoder::FrameDecoder;
pub use self::encoder::FrameEncoder;
pub use self::fragmented::{
    FragmentedFrameCodec,
    FragmentedFrameDecoder,
    FragmentedFrameEncoder,
};

pub const MAX_FRAME_LEN: usize = 16 * 1024 * 1024;

pub type FrameCodec<const MTU: usize = MAX_FRAME_LEN> =
    ComposedCodec<FrameDecoder<MTU>, FrameEncoder<MTU>>;
