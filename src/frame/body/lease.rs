use super::util::{decode_chained, ChainedEncoder};
use super::{codec::BodyCodec, NonZero, RestMetadata};
use crate::error::RSocketResult;
use crate::frame::codec::{ContextDecodable, Encodable};
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone)]
pub struct Lease<'a> {
    pub ttl: NonZero<u32>,
    pub number_of_requests: NonZero<u32>,
    pub metadata: Option<RestMetadata<'a>>,
}

impl<'a> ContextDecodable<'a, &super::BodyDecodeContext> for Lease<'a> {
    fn decode_with(
        input: &'a [u8],
        cx: &super::BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        decode_chained(move |m| {
            Ok(Self {
                ttl: m.next()?,
                number_of_requests: m.next()?,
                metadata: m.next_with(cx)?,
            })
        })(input)
    }
}

impl Encodable for Lease<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer
            .encode(&self.ttl)?
            .encode(&self.number_of_requests)?
            .encode_opt(&self.metadata)
    }
}

impl<'a> BodyCodec<'a> for Lease<'a> {
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::METADATA)?
            .in_stream(0)?
            .done()
    }
}
