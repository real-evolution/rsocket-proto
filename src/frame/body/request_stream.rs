use derive_more::From;
use nom::{combinator::rest, sequence::tuple};

use super::codec::BodyCodec;
use crate::{
    error::RSocketResult,
    frame::{codec, Flags, FrameHeader},
};

#[derive(Debug, Clone, From)]
pub struct RequestStream<'a> {
    pub initial_request_n: u32,
    pub metadata: Option<&'a [u8]>,
    pub data: &'a [u8],
}

impl<'a> BodyCodec<'a> for RequestStream<'a> {
    fn decode(
        input: &'a [u8],
        cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        codec::map_into(tuple((
            codec::non_zero_be_u32,
            codec::length_metadata(cx),
            rest,
        )))(input)
    }

    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::METADATA | Flags::FOLLOW)?
            .done()
    }
}
