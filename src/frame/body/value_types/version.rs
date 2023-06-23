use crate::frame::codec::{Decodable, Encodable};

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

impl Encodable for Version {
    fn encode<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        use byteorder::{WriteBytesExt, BE};

        writer.write_u16::<BE>(self.major)?;
        writer.write_u16::<BE>(self.minor)?;

        Ok(())
    }
}

impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version { major: 1, minor: 0 }
    }
}
