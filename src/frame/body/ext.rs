use derive_more::From;

use super::util::{decode_chained, ChainedEncoder};
use super::{codec::BodyCodec, Data, Number, PrefixedMetadata};
use crate::error::RSocketResult;
use crate::frame::codec::{ContextDecodable, Encodable};
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone, From)]
pub struct Ext<'a> {
    pub extended_type: Number<u32>,
    pub metadata: Option<PrefixedMetadata<'a>>,
    pub data: Data<'a>,
}

impl<'a> ContextDecodable<'a, &super::BodyDecodeContext> for Ext<'a> {
    fn decode_with(
        input: &'a [u8],
        cx: &super::BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        decode_chained(move |m| {
            Ok(Self {
                extended_type: m.next()?,
                metadata: m.next_with(cx)?,
                data: m.next()?,
            })
        })(input)
    }
}

impl Encodable for Ext<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer
            .encode(&self.extended_type)?
            .encode_opt(&self.metadata)?
            .encode(&self.data)
    }
}

impl<'a> BodyCodec<'a> for Ext<'a> {
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::IGNORE | Flags::METADATA)?
            .done()
    }
}
