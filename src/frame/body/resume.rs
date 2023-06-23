use derive_more::From;

use super::{codec::BodyCodec, Number, ResumeToken, Version};
use crate::{
    error::RSocketResult,
    frame::{
        codec::{self, chained},
        FrameHeader,
    },
};

#[derive(Debug, Clone, From)]
pub struct Resume<'a> {
    pub version: Version,
    pub resume_identification_token: ResumeToken<'a>,
    pub last_received_server_position: Number<u64>,
    pub first_available_client_position: Number<u64>,
}

impl<'a> BodyCodec<'a> for Resume<'a> {
    fn decode(
        input: &'a [u8],
        _cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        chained(move |m| {
            Ok(Self {
                version: m.next()?,
                resume_identification_token: m.next()?,
                last_received_server_position: m.next()?,
                first_available_client_position: m.next()?,
            })
        })(input)
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
