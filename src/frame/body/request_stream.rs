use super::util::{decode_chained, ChainedEncoder};
use crate::frame::codec::{ContextDecodable, Encodable};
use crate::frame::Flags;

#[derive(Debug, Clone)]
pub struct RequestStream<'a> {
    pub initial_request_n: super::NonZero<u32>,
    pub metadata: Option<super::PrefixedMetadata<'a>>,
    pub data: super::Data<'a>,
}

impl super::BodySpec for RequestStream<'_> {
    const FLAGS_MASK: Flags = crate::const_flags![METADATA | FOLLOW];
}

impl<'a> ContextDecodable<'a, &super::BodyDecodeContext> for RequestStream<'a> {
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

impl Encodable for RequestStream<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer
            .encode(&self.initial_request_n)?
            .encode_opt(&self.metadata)?
            .encode(&self.data)
    }
}
