use crate::frame::codec::Decodable;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}

impl<'a> Decodable<'a> for Version {
    fn decode(input: &[u8]) -> nom::IResult<&[u8], Version> {
        use nom::number::complete::be_u16;

        let (r, major) = be_u16(input)?;
        let (r, minor) = be_u16(r)?;

        Ok((r, Version { major, minor }))
    }
}

impl Version {
    pub(crate) fn encode<'a, W: std::io::Write>(
        &self,
        writer: &'a mut W,
    ) -> std::io::Result<&'a mut W> {
        use byteorder::{WriteBytesExt, BE};

        writer.write_u16::<BE>(self.major)?;
        writer.write_u16::<BE>(self.minor)?;

        Ok(writer)
    }
}

impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version { major: 1, minor: 0 }
    }
}
