mod encoder;
pub use encoder::FrameEncoder;

pub const MAX_FRAME_LEN: usize = 16 * 1024 * 1024;
