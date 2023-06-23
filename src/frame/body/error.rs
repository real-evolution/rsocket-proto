use super::util::{decode_chained, ChainedEncoder};
use crate::frame::codec::{Decodable, Encodable};

#[derive(Debug, Clone)]
pub struct Error<'a> {
    pub code: super::ErrorCode,
    pub data: super::Utf8Text<'a>,
}

impl super::BodySpec for Error<'_> {
    const FLAGS_MASK: crate::frame::Flags = crate::const_flags![];
}

impl<'a> Decodable<'a> for Error<'a> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        decode_chained(move |m| {
            Ok(Self {
                code: m.next()?,
                data: m.next()?,
            })
        })(input)
    }
}

impl Encodable for Error<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer.encode(&self.code)?.encode(&self.data)
    }
}
