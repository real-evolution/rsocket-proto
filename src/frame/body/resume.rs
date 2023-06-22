use derive_more::From;
use nom::{
    multi::length_data,
    number::complete::{be_u16, be_u64},
    sequence::tuple,
};

use super::{codec::BodyCodec, Version};
use crate::{
    error::RSocketResult,
    frame::{codec, FrameHeader},
};

#[derive(Debug, Clone, From)]
pub struct Resume<'a> {
    pub version: Version,
    pub resume_identification_token: &'a [u8],
    pub last_received_server_position: u64,
    pub first_available_client_position: u64,
}

impl<'a> BodyCodec<'a> for Resume<'a> {
    fn decode(
        input: &'a [u8],
        _cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        codec::map_into(tuple((
            Version::decode,
            length_data(be_u16),
            be_u64,
            be_u64,
        )))(input)
    }

    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().has_empty_flags()?.in_stream(0)?.done()
    }
}
