mod buffer;
mod header;
mod value_types;
mod variant;

pub use buffer::*;
pub use header::*;
pub use value_types::*;
pub use variant::*;

use recode::bytes::{Bytes, BytesMut};
use recode::{util::EncoderExt, Decoder, Encoder};

#[derive(Debug)]
pub struct Frame {
    header: FrameHeader,
    variant: FrameVariant,
}

impl Frame {
    #[inline]
    pub fn header(&self) -> &FrameHeader {
        &self.header
    }

    #[inline]
    pub fn variant(&self) -> &FrameVariant {
        &self.variant
    }
}

impl Decoder<Bytes> for Frame {
    type Error = crate::Error;

    fn decode(buf: &mut Bytes) -> Result<Self, Self::Error> {
        let mut header = FrameHeader::decode(buf)?;
        let mut buf_wrapper = Buffer::new(buf.clone(), header);
        let variant = FrameVariant::decode(&mut buf_wrapper)?;

        (*buf, header) = buf_wrapper.into_parts();

        Ok(Self { header, variant })
    }
}

impl Encoder<BytesMut> for Frame {
    type Error = crate::Error;

    fn encode(item: &Self, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let mut buf_wrapper = BufferMut::new(buf.split(), item.header);

        item.header.encode_to(&mut buf_wrapper)?;
        item.variant.encode_to(&mut buf_wrapper)?;

        (*buf, _) = buf_wrapper.into_parts();

        Ok(())
    }
}
