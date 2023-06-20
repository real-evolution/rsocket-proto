use std::io::Write;

use derive_more::From;
use from_to_repr::from_to_other;
use nom::{
    combinator::{into, map},
    number::complete::be_u32,
    sequence::tuple,
};

use super::{parse::BodyCodec, util::rest_utf8};
use crate::frame::FrameHeader;

#[derive(Debug, Clone, From)]
pub struct Error<'a> {
    pub code: ErrorCode,
    pub data: &'a str,
}

#[derive(Debug, Clone)]
#[from_to_other(base_type = u32)]
pub enum ErrorCode {
    InvalidSetup = 0x00000001,
    UnsupportedSetup = 0x00000002,
    RejectSetup = 0x00000003,
    RejectResume = 0x00000004,
    ConnectionError = 0x00000101,
    ConnectionClose = 0x00000102,
    ApplicationError = 0x00000201,
    Rejected = 0x00000202,
    Canceled = 0x00000203,
    Invalid = 0x00000204,
    Other(u32),
}

impl<'a> BodyCodec<'a> for Error<'a> {
    fn decode(
        _header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        into(tuple((map(be_u32, ErrorCode::from_base_type), rest_utf8)))(input)
    }

    fn encode<W: Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        todo!()
    }
}
