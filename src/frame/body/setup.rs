use super::{ChainedEncoder, ContextDecodable, Encodable};
use crate::frame::Flags;

#[derive(Debug, Clone)]
pub struct Setup<'a> {
    pub version: super::Version,
    pub keepalive: super::NonZero<u32>,
    pub lifetime: super::NonZero<u32>,
    pub token: Option<super::ResumeToken<'a>>,
    pub mime_metadata: super::MimeType<'a>,
    pub mime_data: super::MimeType<'a>,
    pub metadata: Option<super::PrefixedMetadata<'a>>,
    pub data: super::Data<'a>,
}

impl super::BodySpec for Setup<'_> {
    const FLAGS_MASK: Flags = crate::const_flags![METADATA | RESUME | LEASE];
    const IS_CONNECTION_STREAM: bool = true;
}

impl<'a> ContextDecodable<'a, &super::BodyDecodeContext> for Setup<'a> {
    fn decode_with(
        input: &'a [u8],
        cx: &super::BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        super::decode_chained(move |m| {
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
