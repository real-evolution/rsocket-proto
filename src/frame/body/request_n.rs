use derive_more::From;
use nom::{
    combinator::{map, verify},
    number::complete::be_u32,
};

use super::parse::BodyCodec;
use crate::frame::FrameHeader;

#[derive(Debug, Clone, From)]
pub struct RequestN {
    pub request_n: u32,
}

impl<'a> BodyCodec<'a> for RequestN {
    fn decode(
        _header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        map(verify(be_u32, |&v| v > 0), Into::into)(input)
    }

    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }
}
