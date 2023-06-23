use super::{Decodable, Encodable};

#[derive(Debug, Clone)]
pub struct Cancel;

impl super::BodySpec for Cancel {
    const FLAGS_MASK: crate::frame::Flags = crate::const_flags![];
}

impl<'a> Decodable<'a> for Cancel {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        Ok((input, Self))
    }
}

impl Encodable for Cancel {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        Ok(writer)
    }
}
