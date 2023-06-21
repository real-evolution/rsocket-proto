use std::io::Write;

use derive_more::From;
use nom::{
    combinator::{cond, rest},
    sequence::tuple,
};

use super::codec::BodyCodec;
use crate::{
    error::RSocketResult,
    frame::{codec, Flags, FrameHeader},
};

#[derive(Debug, Clone, From)]
pub struct Lease<'a> {
    pub ttl: u32,
    pub number_of_requests: u32,
    pub metadata: Option<&'a [u8]>,
}

impl<'a> BodyCodec<'a> for Lease<'a> {
    fn decode(
        input: &'a [u8],
        cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        codec::map_into(tuple((
            codec::non_zero_be_u32,
            codec::non_zero_be_u32,
            cond(cx.header.flags.contains(Flags::METADATA), rest),
        )))(input)
    }

    fn encode<W: Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        todo!()
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::METADATA)?
            .in_stream(0)?
            .done()
    }
}
