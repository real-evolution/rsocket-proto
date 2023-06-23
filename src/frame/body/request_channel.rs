use super::{ChainedEncoder, ContextDecodable, Encodable};
use crate::frame::Flags;

#[derive(Debug, Clone)]
pub struct RequestChannel<'a> {
    pub initial_request_n: super::NonZero<u32>,
    pub metadata: Option<super::PrefixedMetadata<'a>>,
    pub data: super::Data<'a>,
}

impl super::BodySpec for RequestChannel<'_> {
    const FLAGS_MASK: Flags = crate::const_flags![METADATA | FOLLOW | COMPLETE];
}

impl<'a> ContextDecodable<'a, &super::BodyDecodeContext>
    for RequestChannel<'a>
{
    fn decode_with(
        input: &'a [u8],
        cx: &super::BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        super::decode_chained(move |m| {
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
