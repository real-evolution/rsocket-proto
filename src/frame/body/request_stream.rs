use super::util::chained;
use super::{codec::BodyCodec, Data, NonZero, PrefixedMetadata};
use crate::error::RSocketResult;
use crate::frame::codec;
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone)]
pub struct RequestStream<'a> {
    pub initial_request_n: NonZero<u32>,
    pub metadata: Option<PrefixedMetadata<'a>>,
    pub data: Data<'a>,
}

impl<'a> BodyCodec<'a> for RequestStream<'a> {
    fn decode(
        input: &'a [u8],
        cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        chained(move |m| {
            Ok(Self {
                initial_request_n: m.next()?,
                metadata: m.next_with(cx)?,
                data: m.next()?,
            })
        })(input)
    }

    fn encode<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.initial_request_n.encode(writer)?;

        if let Some(metadata) = &self.metadata {
            metadata.encode(writer)?;
        }

        self.data.encode(writer)?;

        Ok(())
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::METADATA | Flags::FOLLOW)?
            .done()
    }
}
