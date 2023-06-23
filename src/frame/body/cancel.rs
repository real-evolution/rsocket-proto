use super::codec::EmptyBody;
use crate::error::RSocketResult;
use crate::frame::codec::Encodable;
use crate::frame::{codec::Decodable, FrameHeader};

#[derive(Debug, Clone)]
pub struct Cancel;

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

impl EmptyBody for Cancel {
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().has_empty_flags()?.done()
    }
}
