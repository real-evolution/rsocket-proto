use derive_more::From;
use nom::{
    combinator::into,
    multi::length_data,
    number::complete::{be_u16, be_u64},
    sequence::tuple,
};

use super::{codec::BodyCodec, Version};
use crate::frame::FrameHeader;

#[derive(Debug, Clone, From)]
pub struct Resume<'a> {
    pub version: Version,
    pub resume_identification_token: &'a [u8],
    pub last_received_server_position: u64,
    pub first_available_client_position: u64,
}

impl<'a> BodyCodec<'a> for Resume<'a> {
    fn decode(
        _header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        into(tuple((Version::parse, length_data(be_u16), be_u64, be_u64)))(
            input,
        )
    }

    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }
}
