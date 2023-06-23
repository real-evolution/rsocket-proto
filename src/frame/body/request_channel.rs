use super::codec::BodyCodec;
use super::{Data, NonZero, PrefixedMetadata};
use crate::error::RSocketResult;
use crate::frame::codec::chained;
use crate::frame::{codec, Flags, FrameHeader};

#[derive(Debug, Clone)]
pub struct RequestChannel<'a> {
    pub initial_request_n: NonZero<u32>,
    pub metadata: Option<PrefixedMetadata<'a>>,
    pub data: Data<'a>,
}

impl<'a> BodyCodec<'a> for RequestChannel<'a> {
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

    fn encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::io::Result<()> {
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
            .flags_match_mask(
                Flags::METADATA | Flags::FOLLOW | Flags::COMPLETE,
            )?
            .done()
    }
}
