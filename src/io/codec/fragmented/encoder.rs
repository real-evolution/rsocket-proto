use tokio_util::codec::Encoder;

use crate::frame::TaggedFrame;
use crate::io::codec::FrameEncoder;
use crate::io::Fragmenter;

#[derive(Debug, Default, Clone, Copy)]
pub struct FragmentedFrameEncoder<const MTU: usize>;

impl<const MTU: usize> Encoder<TaggedFrame> for FragmentedFrameEncoder<MTU> {
    type Error = crate::Error;

    #[inline]
    fn encode(
        &mut self,
        item: TaggedFrame,
        dst: &mut recode::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        for f in Fragmenter::<MTU>::fragment(item) {
            FrameEncoder.encode(f, dst)?;
        }

        Ok(())
    }
}
