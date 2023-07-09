use recode::{bytes::Buf, codec::u24, Decoder as _};
use tokio_util::codec::Decoder;

use crate::frame::Frame;

#[derive(Debug, Clone, Copy, Default)]
enum FrameDecoderState {
    #[default]
    Header,
    Payload(usize),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FrameDecoder<const MTU: usize = { super::MAX_FRAME_LEN }> {
    state: FrameDecoderState,
}

impl<const MTU: usize> Decoder for FrameDecoder<MTU> {
    type Item = Frame;
    type Error = crate::Error;

    fn decode(
        &mut self,
        src: &mut recode::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        match self.state {
            | FrameDecoderState::Header => {
                let len = u24::decode(src)?;

                self.state = FrameDecoderState::Payload(len);
                self.decode(src)
            }
            | FrameDecoderState::Payload(len) => {
                if src.remaining() < len {
                    return Ok(None);
                }

                let mut buf = src.split_to(len).freeze();
                let frame = Frame::decode(&mut buf)?;

                Ok(Some(frame))
            }
        }
    }
}
