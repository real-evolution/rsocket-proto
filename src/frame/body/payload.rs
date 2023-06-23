use derive_more::From;

use super::util::chained;
use super::{codec::BodyCodec, Data, PrefixedMetadata};
use crate::error::RSocketResult;
use crate::frame::codec::{ContextDecodable, Encodable};
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone, From)]
pub struct Payload<'a> {
    pub metadata: Option<PrefixedMetadata<'a>>,
    pub data: Option<Data<'a>>,
}

impl<'a> ContextDecodable<'a, &super::BodyDecodeContext> for Payload<'a> {
    fn decode_with(
        input: &'a [u8],
        cx: &super::BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        chained(move |m| {
            Ok(Self {
                metadata: m.next_with(cx)?,
                data: m.next_with(cx)?,
            })
        })(input)
    }
}

impl Encodable for Payload<'_> {
    fn encode<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        if let Some(metadata) = &self.metadata {
            metadata.encode(writer)?;
        }

        if let Some(data) = &self.data {
            data.encode(writer)?;
        }

        Ok(())
    }
}

impl<'a> BodyCodec<'a> for Payload<'a> {
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(
                Flags::METADATA | Flags::FOLLOW | Flags::COMPLETE | Flags::NEXT,
            )?
            .done()
    }
}
