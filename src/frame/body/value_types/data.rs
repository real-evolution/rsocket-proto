use derive_more::Deref;

use crate::frame::codec::{ContextDecodable, Decodable, Encodable};
use crate::frame::{BodyDecodeContext, Flags};

#[derive(Debug, Clone, Deref)]
#[repr(transparent)]
pub struct Data<'a>(&'a [u8]);

impl<'a> Decodable<'a> for Data<'a> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        use nom::combinator::rest;

        rest(input).map(|(r, o)| (r, Self(o)))
    }
}

impl Encodable for Data<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer.write_all(self.0)?;

        Ok(writer)
    }
}

impl<'a> ContextDecodable<'a, &BodyDecodeContext> for Option<Data<'a>> {
    fn decode_with(
        input: &'a [u8],
        cx: &BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        let flags = cx.header.flags;

        if flags.contains(Flags::NEXT) {
            return Ok((input, None));
        }

        let (rest, data) = Decodable::decode(input)?;

        Ok((rest, Some(data)))
    }
}
