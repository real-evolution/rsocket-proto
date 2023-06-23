use super::{ChainedEncoder, ContextDecodable, Encodable};
use crate::frame::Flags;

#[derive(Debug, Clone)]
pub struct Ext<'a> {
    pub extended_type: super::Number<u32>,
    pub metadata: Option<super::PrefixedMetadata<'a>>,
    pub data: super::Data<'a>,
}

impl super::BodySpec for Ext<'_> {
    const FLAGS_MASK: Flags = crate::const_flags![IGNORE | METADATA];
}

impl<'a> ContextDecodable<'a, &super::BodyDecodeContext> for Ext<'a> {
    fn decode_with(
        input: &'a [u8],
        cx: &super::BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        super::decode_chained(move |m| {
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
