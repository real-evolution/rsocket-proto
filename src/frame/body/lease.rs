use super::util::{decode_chained, ChainedEncoder};
use crate::frame::codec::{ContextDecodable, Encodable};

#[derive(Debug, Clone)]
pub struct Lease<'a> {
    pub ttl: super::NonZero<u32>,
    pub number_of_requests: super::NonZero<u32>,
    pub metadata: Option<super::RestMetadata<'a>>,
}

impl super::BodySpec for Lease<'_> {
    const FLAGS_MASK: crate::frame::Flags = crate::const_flags![METADATA];
    const IS_CONNECTION_STREAM: bool = true;
}

impl<'a> ContextDecodable<'a, &super::BodyDecodeContext> for Lease<'a> {
    fn decode_with(
        input: &'a [u8],
        cx: &super::BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        decode_chained(move |m| {
            Ok(Self {
                ttl: m.next()?,
                number_of_requests: m.next()?,
                metadata: m.next_with(cx)?,
            })
        })(input)
    }
}

impl Encodable for Lease<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer
            .encode(&self.ttl)?
            .encode(&self.number_of_requests)?
            .encode_opt(&self.metadata)
    }
}
