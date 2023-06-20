use std::io::Write;

use derive_more::From;
use nom::{
    combinator::{into, verify},
    number::complete::be_u64,
    sequence::tuple,
};

use super::{codec::BodyCodec, util::rest_opt};
use crate::{
    error::RSocketResult,
    frame::{Flags, FrameHeader},
};

#[derive(Debug, Clone, From)]
pub struct Keepalive<'a> {
    pub last_received_position: u64,
    pub data: Option<&'a [u8]>,
}

impl<'a> BodyCodec<'a> for Keepalive<'a> {
    fn decode(
        _header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        into(tuple((verify(be_u64, |&v| v > 0), rest_opt)))(input)
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
