use super::util::{decode_chained, ChainedEncoder};
use crate::frame::codec::{ContextDecodable, Encodable};
use crate::frame::Flags;

#[derive(Debug, Clone)]
pub struct RequestFNF<'a> {
    pub metadata: Option<super::PrefixedMetadata<'a>>,
    pub data: super::Data<'a>,
}

impl super::BodySpec for RequestFNF<'_> {
    const FLAGS_MASK: Flags = crate::const_flags![METADATA | FOLLOW];
}

impl<'a> ContextDecodable<'a, &super::BodyDecodeContext> for RequestFNF<'a> {
    fn decode_with(
        input: &'a [u8],
        cx: &super::BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        decode_chained(move |m| {
            Ok(Self {
                metadata: m.next_with(cx)?,
                data: m.next()?,
            })
        })(input)
    }
}

impl Encodable for RequestFNF<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer.encode_opt(&self.metadata)?.encode(&self.data)
    }
}
