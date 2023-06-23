use super::codec::BodyCodec;
use super::util::{decode_chained, ChainedEncoder};
use super::{Data, NonZero, PrefixedMetadata};
use crate::error::RSocketResult;
use crate::frame::codec::{ContextDecodable, Encodable};
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone)]
pub struct RequestChannel<'a> {
    pub initial_request_n: NonZero<u32>,
    pub metadata: Option<PrefixedMetadata<'a>>,
    pub data: Data<'a>,
}

impl<'a> ContextDecodable<'a, &super::BodyDecodeContext>
    for RequestChannel<'a>
{
    fn decode_with(
        input: &'a [u8],
        cx: &super::BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        decode_chained(move |m| {
            Ok(Self {
                initial_request_n: m.next()?,
                metadata: m.next_with(cx)?,
                data: m.next()?,
            })
        })(input)
    }
}

impl Encodable for RequestChannel<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer.encode_opt(&self.metadata)?.encode(&self.data)
    }
}

impl<'a> BodyCodec<'a> for RequestChannel<'a> {
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(
                Flags::METADATA | Flags::FOLLOW | Flags::COMPLETE,
            )?
            .done()
    }
}
