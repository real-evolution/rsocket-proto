use std::io::Write;

use derive_more::From;
use nom::{combinator::rest, sequence::tuple};

use super::codec::BodyCodec;
use crate::{
    error::RSocketResult,
    frame::{codec, Flags, FrameHeader},
};

#[derive(Debug, Clone, From)]
pub struct Keepalive<'a> {
    pub last_received_position: u64,
    pub data: Option<&'a [u8]>,
}

impl<'a> BodyCodec<'a> for Keepalive<'a> {
    fn decode(
        input: &'a [u8],
        _cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        codec::map_into(tuple((
            codec::non_zero_be_u64,
            codec::none_if_empty(rest),
        )))(input)
    }

    fn encode<W: Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        todo!()
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::RESPOND)?
            .in_stream(0)?
            .done()
    }
}
