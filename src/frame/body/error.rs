use super::util::{decode_chained, ChainedEncoder};
use super::{codec::BodyCodec, ErrorCode, Utf8Text};
use crate::error::RSocketResult;
use crate::frame::codec::{Decodable, Encodable};
use crate::frame::FrameHeader;

#[derive(Debug, Clone)]
pub struct Error<'a> {
    pub code: ErrorCode,
    pub data: Utf8Text<'a>,
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

impl<'a> BodyCodec<'a> for Error<'a> {
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().has_empty_flags()?.done()
    }
}
