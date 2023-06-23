use derive_more::Deref;

use crate::frame::codec::{ContextDecodable, Decodable, ParseContext};
use crate::frame::Flags;

#[derive(Debug, Clone, Deref)]
#[repr(transparent)]
pub struct ResumeToken<'a>(&'a [u8]);

impl<'a> Decodable<'a> for ResumeToken<'a> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        use nom::multi::length_data;
        use nom::number::complete::be_u16;

        let (r, token) = length_data(be_u16)(input)?;

        Ok((r, Self(token)))
    }
}

impl<'a> ContextDecodable<'a, &ParseContext<'a>> for Option<ResumeToken<'a>> {
    fn decode_with(
        input: &'a [u8],
        cx: &ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        if cx.header.flags.contains(Flags::RESUME) {
            return Ok((input, None));
        }

        let (rest, token) = ResumeToken::decode(input)?;

        Ok((rest, Some(token)))
    }
}

impl<'a> ResumeToken<'a> {
    pub(crate) fn encode<'b, W: std::io::Write>(
        &self,
        writer: &'b mut W,
    ) -> std::io::Result<&'b mut W> {
        use byteorder::{WriteBytesExt, BE};

        writer.write_u16::<BE>(self.0.len() as u16)?;
        writer.write_all(self.0)?;

        Ok(writer)
    }
}
