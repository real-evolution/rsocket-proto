use derive_more::From;

use super::codec::BodyCodec;
use crate::{
    error::RSocketResult,
    frame::{codec, FrameHeader},
};

#[derive(Debug, Clone, From)]
pub struct RequestN {
    pub request_n: u32,
}

impl<'a> BodyCodec<'a> for RequestN {
    fn decode(
        input: &'a [u8],
        _cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        codec::map_into(codec::non_zero_be_u32)(input)
    }

    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().has_empty_flags()?.done()
    }
}
