use std::io::Write;

use derive_more::From;
use nom::{
    combinator::{cond, map, rest, verify},
    number::complete::be_u32,
    sequence::tuple,
};

use super::codec::BodyCodec;
use crate::{
    error::RSocketResult,
    frame::{Flags, FrameHeader},
};

#[derive(Debug, Clone, From)]
pub struct Lease<'a> {
    pub ttl: u32,
    pub number_of_requests: u32,
    pub metadata: Option<&'a [u8]>,
}

impl<'a> BodyCodec<'a> for Lease<'a> {
    fn decode(
        header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        map(
            tuple((
                verify(be_u32, |&v| v > 0),
                verify(be_u32, |&v| v > 0),
                cond(header.flags.contains(Flags::METADATA), rest),
            )),
            Into::into,
        )(input)
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
