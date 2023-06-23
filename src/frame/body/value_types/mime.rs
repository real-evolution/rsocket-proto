use derive_more::Deref;

use crate::frame::codec::Decodable;

#[derive(Debug, Clone, Deref)]
#[repr(transparent)]
pub struct MimeType<'a>(&'a str);

impl<'a> Decodable<'a> for MimeType<'a> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        use nom::combinator::map_res;
        use nom::multi::length_data;
        use nom::number::complete::be_u8;

        let (r, mime) =
            map_res(length_data(be_u8), std::str::from_utf8)(input)?;

        Ok((r, Self(mime)))
    }
}

impl<'a> MimeType<'a> {
    pub(crate) fn encode<'b, W: std::io::Write>(
        &self,
        writer: &'b mut W,
    ) -> std::io::Result<&'b mut W> {
        use byteorder::WriteBytesExt;

        let buf = self.0.as_bytes();

        writer.write_u8(buf.len() as u8)?;
        writer.write_all(buf)?;

        Ok(writer)
    }
}
