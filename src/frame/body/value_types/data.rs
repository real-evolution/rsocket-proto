use derive_more::Deref;

use crate::frame::{
    codec::{ContextDecodable, Decodable, ParseContext},
    Flags,
};

#[derive(Debug, Clone, Deref)]
#[repr(transparent)]
pub struct Data<'a>(&'a [u8]);

impl<'a> Data<'a> {
    pub(crate) fn encode<'b, W: std::io::Write>(
        &self,
        writer: &'b mut W,
    ) -> std::io::Result<&'b mut W> {
        writer.write_all(self.0)?;

        Ok(writer)
    }
}

impl<'a> Decodable<'a> for Data<'a> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        use nom::combinator::rest;

        rest(input).map(|(r, o)| (r, Self(o)))
    }
}

impl<'a> ContextDecodable<'a, &ParseContext<'a>> for Option<Data<'a>> {
    fn decode_with(
        input: &'a [u8],
        cx: &ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        let flags = cx.header.flags;

        if flags.contains(Flags::NEXT) {
            return Ok((input, None));
        }

        let (rest, data) = Decodable::decode(input)?;

        Ok((rest, Some(data)))
    }
}
