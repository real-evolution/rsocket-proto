use derive_more::Deref;

use crate::frame::{codec, Flags};

#[derive(Debug, Deref)]
#[repr(transparent)]
pub struct ResumeToken<'a>(&'a [u8]);

impl<'a> ResumeToken<'a> {
    pub(crate) fn decode_opt(
        cx: &codec::ParseContext<'a>,
    ) -> impl FnMut(&'a [u8]) -> nom::IResult<&'a [u8], Option<ResumeToken<'a>>> + 'a
    {
        let flags = cx.header.flags;

        move |input| {
            if flags.contains(Flags::RESUME) {
                return Ok((input, None));
            }

            let (rest, token) = Self::decode(input)?;

            Ok((rest, Some(token)))
        }
    }

    pub(crate) fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        use nom::multi::length_data;
        use nom::number::complete::be_u16;

        let (r, token) = length_data(be_u16)(input)?;

        Ok((r, Self(token)))
    }

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
