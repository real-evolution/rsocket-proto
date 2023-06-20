use derive_more::From;
use nom::{
    combinator::{into, rest, verify},
    number::complete::be_u32,
    sequence::tuple,
};

use super::{parse::BodyCodec, util::metadata_opt};
use crate::frame::FrameHeader;

#[derive(Debug, Clone, From)]
pub struct RequestChannel<'a> {
    pub initial_request_n: u32,
    pub metadata: Option<&'a [u8]>,
    pub data: &'a [u8],
}

impl<'a> BodyCodec<'a> for RequestChannel<'a> {
    fn decode(
        header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        into(tuple((
            verify(be_u32, |&v| v > 0),
            metadata_opt(header),
            rest,
        )))(input)
    }

    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }
}
