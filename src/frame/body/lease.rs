use std::io::Write;

use derive_more::From;

use super::{codec::BodyCodec, NonZero, RestMetadata};
use crate::frame::{codec, Flags, FrameHeader};
use crate::{error::RSocketResult, frame::codec::chained};

#[derive(Debug, Clone, From)]
pub struct Lease<'a> {
    pub ttl: NonZero<u32>,
    pub number_of_requests: NonZero<u32>,
    pub metadata: Option<RestMetadata<'a>>,
}

impl<'a> BodyCodec<'a> for Lease<'a> {
    fn decode(
        input: &'a [u8],
        cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        chained(move |m| {
            Ok(Self {
                ttl: m.next()?,
                number_of_requests: m.next()?,
                metadata: m.next_with(cx)?,
            })
        })(input)
    }

    fn encode<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.ttl.encode(writer)?;
        self.number_of_requests.encode(writer)?;
        if let Some(metadata) = &self.metadata {
            metadata.encode(writer)?;
        }

        Ok(())
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::METADATA)?
            .in_stream(0)?
            .done()
    }
}
