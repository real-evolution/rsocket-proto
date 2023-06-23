use super::codec::BodyCodec;
use super::util::chained;
use super::{Data, PrefixedMetadata};
use crate::error::RSocketResult;
use crate::frame::codec::ContextDecodable;
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone)]
pub struct RequestFNF<'a> {
    pub metadata: Option<PrefixedMetadata<'a>>,
    pub data: Data<'a>,
}

impl<'a> ContextDecodable<'a, &super::BodyDecodeContext> for RequestFNF<'a> {
    fn decode_with(
        input: &'a [u8],
        cx: &super::BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        chained(move |m| {
            Ok(Self {
                metadata: m.next_with(cx)?,
                data: m.next()?,
            })
        })(input)
    }
}

impl<'a> BodyCodec<'a> for RequestFNF<'a> {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
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
