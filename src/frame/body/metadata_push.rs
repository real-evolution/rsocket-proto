use derive_more::From;

use super::{codec::BodyCodec, RestMetadata};
use crate::error::RSocketResult;
use crate::frame::codec::{self, Decodable};
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone, From)]
pub struct MetadataPush<'a> {
    pub metadata: RestMetadata<'a>,
}

impl<'a> BodyCodec<'a> for MetadataPush<'a> {
    fn decode(
        input: &'a [u8],
        _cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        let (rem, metadata) = Decodable::decode(input)?;

        Ok((rem, Self { metadata }))
    }

    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().flag_is(Flags::METADATA, true)?.done()
    }
}
