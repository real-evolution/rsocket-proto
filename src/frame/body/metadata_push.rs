use derive_more::From;

use super::{codec::BodyCodec, RestMetadata};
use crate::error::RSocketResult;
use crate::frame::codec::Decodable;
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone, From)]
pub struct MetadataPush<'a> {
    pub metadata: RestMetadata<'a>,
}

impl<'a> Decodable<'a> for MetadataPush<'a> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        let (rem, metadata) = Decodable::decode(input)?;

        Ok((rem, Self { metadata }))
    }
}

impl<'a> BodyCodec<'a> for MetadataPush<'a> {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.metadata.encode(writer)?;

        Ok(())
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().flag_is(Flags::METADATA, true)?.done()
    }
}
