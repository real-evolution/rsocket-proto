use recode::bytes::Buf;
use recode::codec::u24;
use recode::Decoder as _;
use tokio_util::codec::Decoder;

use crate::frame::Frame;

#[derive(Clone, Copy, Debug, Default)]
pub enum FrameDecoder {
    #[default]
    Header,
    Payload(usize),
}

impl Decoder for FrameDecoder {
    type Error = crate::Error;
    type Item = Frame;

    fn decode(
        &mut self,
        src: &mut recode::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        match self {
            | Self::Header => {
                let len = u24::decode(src)?;

                *self = Self::Payload(len);
                self.decode(src)
            }
            | Self::Payload(len) => {
                if src.remaining() < *len {
                    return Ok(None);
                }

                let mut buf = src.split_to(*len).freeze();
                let frame = Frame::decode(&mut buf)?;

                *self = Self::Header;

                Ok(Some(frame))
            }
        }
    }
}
