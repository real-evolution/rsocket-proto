use derive_more::From;
use nom::number::complete::be_u64;

use super::codec::BodyCodec;
use crate::{
    error::RSocketResult,
    frame::{codec, FrameHeader},
};

#[derive(Debug, Clone, From)]
pub struct ResumeOk {
    pub last_received_client_position: u64,
}

impl<'a> BodyCodec<'a> for ResumeOk {
    fn decode(
        input: &'a [u8],
        _cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        codec::map_into(be_u64)(input)
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
