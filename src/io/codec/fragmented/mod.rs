mod decoder;
mod encoder;

pub use self::decoder::FragmentedFrameDecoder;
pub use self::encoder::FragmentedFrameEncoder;

pub type FragmentedFrameCodec<const MTU: usize> = super::ComposedCodec<
    FragmentedFrameDecoder<MTU>,
    FragmentedFrameEncoder<MTU>,
>;
