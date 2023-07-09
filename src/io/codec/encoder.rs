use recode::{codec::u24, util::EncoderExt};
use tokio_util::codec::Encoder;

use crate::frame::Frame;

#[derive(Debug, Default, Clone, Copy)]
pub struct FrameEncoder<const MTU: usize = { super::MAX_FRAME_LEN }>;

impl<const MTU: usize> Encoder<Frame> for FrameEncoder<MTU> {
    type Error = crate::Error;

    fn encode(
        &mut self,
        item: Frame,
        dst: &mut recode::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let frame_len = item.size(dst);

        if frame_len <= MTU {
            if let Ok(frame_len) = TryInto::<u24>::try_into(frame_len) {
                dst.reserve(frame_len.size(dst) + item.size(dst));

                frame_len.encode_to(dst)?;
                item.encode_to(dst)?;
            }
        }

        Err(Self::Error::TooLargeFrame {
            frame_type: item.header().frame_type(),
            size: frame_len,
            max_size: MTU,
        })
    }
}
