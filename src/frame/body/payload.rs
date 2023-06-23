use derive_more::From;

use super::util::{decode_chained, ChainedEncoder};
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
        decode_chained(move |m| {
            Ok(Self {
                metadata: m.next_with(cx)?,
                data: m.next_with(cx)?,
            })
        })(input)
    }
}

impl Encodable for Payload<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer.encode_opt(&self.metadata)?.encode_opt(&self.data)
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
