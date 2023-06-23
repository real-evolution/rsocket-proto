use super::util::chained;
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
        chained(move |m| {
            Ok(Self {
                ttl: m.next()?,
                number_of_requests: m.next()?,
                metadata: m.next_with(cx)?,
            })
        })(input)
    }
}

impl Encodable for Lease<'_> {
    fn encode<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.ttl.encode(writer)?;
        self.number_of_requests.encode(writer)?;
        if let Some(metadata) = &self.metadata {
            metadata.encode(writer)?;
        }

        Ok(())
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
