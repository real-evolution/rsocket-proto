mod builder;
mod header;
mod value_types;
mod variant;

pub use builder::*;
pub use header::*;
pub use value_types::*;
pub use variant::*;

use recode::bytes::{Bytes, BytesMut};
use recode::{util::EncoderExt, Decoder, Encoder};

type Buffer = recode::util::ContextBuffer<Bytes, FrameHeader>;
type BufferMut = recode::util::ContextBuffer<BytesMut, FrameHeader>;

#[derive(Debug)]
pub struct Frame {
    header: FrameHeader,
    variant: FrameVariant,
}

impl Frame {
    #[inline]
    pub(crate) fn new(header: FrameHeader, variant: FrameVariant) -> Self {
        Self { header, variant }
    }

    #[inline]
    pub fn header(&self) -> &FrameHeader {
        &self.header
    }

    #[inline]
    pub fn variant(&self) -> &FrameVariant {
        &self.variant
    }

    #[inline]
    pub(crate) fn variant_mut(&mut self) -> &mut FrameVariant {
        &mut self.variant
    }

    #[inline]
    pub const fn builder() -> FrameBuilder {
        FrameBuilder(())
    }

    #[inline]
    pub(crate) fn split(self) -> (FrameHeader, FrameVariant) {
        (self.header, self.variant)
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

    #[inline]
    fn size_of(item: &Self, buf: &BytesMut) -> usize {
        let buf_wrapper = BufferMut::new(buf.clone(), item.header);

        item.header().size(buf) + item.variant().size(&buf_wrapper)
    }
}
