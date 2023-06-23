use super::codec::BodyCodec;
use super::util::{decode_chained, ChainedEncoder};
use super::{Data, MimeType, NonZero, PrefixedMetadata, ResumeToken, Version};
use crate::error::RSocketResult;
use crate::frame::codec::{ContextDecodable, Encodable};
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone)]
pub struct Setup<'a> {
    pub version: Version,
    pub keepalive: NonZero<u32>,
    pub lifetime: NonZero<u32>,
    pub token: Option<ResumeToken<'a>>,
    pub mime_metadata: MimeType<'a>,
    pub mime_data: MimeType<'a>,
    pub metadata: Option<PrefixedMetadata<'a>>,
    pub data: Data<'a>,
}

impl<'a> ContextDecodable<'a, &super::BodyDecodeContext> for Setup<'a> {
    fn decode_with(
        input: &'a [u8],
        cx: &super::BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        decode_chained(move |m| {
            Ok(Self {
                version: m.next()?,
                keepalive: m.next()?,
                lifetime: m.next()?,
                token: m.next_with(cx)?,
                mime_metadata: m.next()?,
                mime_data: m.next()?,
                metadata: m.next_with(cx)?,
                data: m.next()?,
            })
        })(input)
    }
}

impl Encodable for Setup<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer
            .encode(&self.version)?
            .encode(&self.keepalive)?
            .encode(&self.lifetime)?
            .encode_opt(&self.token)?
            .encode(&self.mime_metadata)?
            .encode(&self.mime_data)?
            .encode_opt(&self.metadata)?
            .encode(&self.data)
    }
}

impl<'a> BodyCodec<'a> for Setup<'a> {
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::METADATA | Flags::RESUME | Flags::LEASE)?
            .in_stream(0)?
            .done()
    }
}
