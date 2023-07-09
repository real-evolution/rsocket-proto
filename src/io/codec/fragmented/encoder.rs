use tokio_util::codec::Encoder;

use crate::frame::Frame;
use crate::io::{codec::FrameEncoder, Fragmenter};

#[derive(Debug, Default, Clone, Copy)]
pub struct FragmentedFrameEncoder<const MTU: usize>;

impl<const MTU: usize> Encoder<Frame> for FragmentedFrameEncoder<MTU> {
    type Error = crate::Error;

    #[inline]
    fn encode(
        &mut self,
        item: Frame,
        dst: &mut recode::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        for f in Fragmenter::<MTU>::fragment(item) {
            FrameEncoder::<MTU>.encode(f, dst)?;
        }

        Ok(())
    }
}
