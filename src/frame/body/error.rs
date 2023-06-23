use std::io::Write;

use super::util::chained;
use super::{codec::BodyCodec, ErrorCode, Utf8Text};
use crate::error::RSocketResult;
use crate::frame::codec;
use crate::frame::FrameHeader;

#[derive(Debug, Clone)]
pub struct Error<'a> {
    pub code: ErrorCode,
    pub data: Utf8Text<'a>,
}

impl<'a> BodyCodec<'a> for Error<'a> {
    fn decode(
        input: &'a [u8],
        _cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        chained(move |m| {
            Ok(Self {
                code: m.next()?,
                data: m.next()?,
            })
        })(input)
    }

    fn encode<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.code.encode(writer)?;
        self.data.encode(writer)?;

        Ok(())
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().has_empty_flags()?.done()
    }
}
