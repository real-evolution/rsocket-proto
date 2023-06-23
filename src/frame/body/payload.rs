use super::util::{decode_chained, ChainedEncoder};
use crate::frame::codec::{ContextDecodable, Encodable};

#[derive(Debug, Clone)]
pub struct Payload<'a> {
    pub metadata: Option<super::PrefixedMetadata<'a>>,
    pub data: Option<super::Data<'a>>,
}

impl super::BodySpec for Payload<'_> {
    const FLAGS_MASK: crate::frame::Flags =
        crate::const_flags![METADATA | FOLLOW | COMPLETE | NEXT];
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
