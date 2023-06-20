use std::io::Write;

use derive_more::From;
use nom::{
    combinator::{into, verify},
    number::complete::be_u32,
    sequence::tuple,
};

use super::{codec::BodyCodec, util::rest_opt};
use crate::frame::FrameHeader;

#[derive(Debug, Clone, From)]
pub struct Lease<'a> {
    pub ttl: u32,
    pub number_of_requests: u32,
    pub metadata: Option<&'a [u8]>,
}

impl<'a> BodyCodec<'a> for Lease<'a> {
    fn decode(
        _header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        into(tuple((
            verify(be_u32, |&v| v > 0),
            verify(be_u32, |&v| v > 0),
            rest_opt,
        )))(input)
    }

    fn encode<W: Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        todo!()
    }
}
