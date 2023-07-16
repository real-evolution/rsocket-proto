use recode::codec::u24;
use recode::util::EncoderExt;
use tokio_util::codec::Encoder;

use crate::frame::TaggedFrame;

#[derive(Debug, Default, Clone, Copy)]
pub struct FrameEncoder;

impl Encoder<TaggedFrame> for FrameEncoder {
    type Error = crate::Error;

    fn encode(
        &mut self,
        item: TaggedFrame,
        dst: &mut recode::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let frame_len = item.size(dst);

        if frame_len <= super::MAX_FRAME_LEN {
            if let Ok(len_prefix) = TryInto::<u24>::try_into(frame_len) {
                dst.reserve(len_prefix.size(dst) + frame_len);

                len_prefix.encode_to(dst)?;
                item.encode_to(dst)?;

                return Ok(());
            }
        }

        Err(Self::Error::TooLargeFrame {
            frame_type: item.frame().header().frame_type(),
            size: frame_len,
            max_size: super::MAX_FRAME_LEN,
        })
    }
}
