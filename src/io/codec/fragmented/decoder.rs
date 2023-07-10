use tokio_util::codec::Decoder;

use crate::frame::Frame;
use crate::io::codec::FrameDecoder;
use crate::io::Defragmenter;

#[derive(Debug, Default)]
pub struct FragmentedFrameDecoder<const MTU: usize> {
    decoder: FrameDecoder<MTU>,
    defragmenter: Defragmenter<MTU>,
}

impl<const MTU: usize> Decoder for FragmentedFrameDecoder<MTU> {
    type Error = crate::Error;
    type Item = Frame;

    #[inline]
    fn decode(
        &mut self,
        src: &mut recode::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        while let Some(frame) = self.decoder.decode(src)? {
            if let Some(frame) = self.defragmenter.defragment(frame)? {
                return Ok(Some(frame));
            }
        }

        Ok(None)
    }
}
