use super::{codec::BodyCodec, RestMetadata};
use crate::error::RSocketResult;
use crate::frame::codec::{Decodable, Encodable};
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone)]
pub struct MetadataPush<'a> {
    pub metadata: RestMetadata<'a>,
}

impl<'a> Decodable<'a> for MetadataPush<'a> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        let (rem, metadata) = Decodable::decode(input)?;

        Ok((rem, Self { metadata }))
    }
}

impl Encodable for MetadataPush<'_> {
    fn encode<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.metadata.encode(writer)?;

        Ok(())
    }
}

impl<'a> BodyCodec<'a> for MetadataPush<'a> {
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().flag_is(Flags::METADATA, true)?.done()
    }
}
