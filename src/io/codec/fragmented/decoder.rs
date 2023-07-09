use tokio_util::codec::Decoder;

use crate::{
    frame::Frame,
    io::{codec::FrameDecoder, Defragmenter},
};

#[derive(Debug, Default)]
pub struct FragmentedFrameDecoder<const MTU: usize> {
    decoder: FrameDecoder<MTU>,
    defragmenter: Defragmenter<MTU>,
}

impl<const MTU: usize> Decoder for FragmentedFrameDecoder<MTU> {
    type Item = Frame;
    type Error = crate::Error;

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
