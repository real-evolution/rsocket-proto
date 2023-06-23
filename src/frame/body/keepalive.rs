use std::io::Write;

use derive_more::From;

use super::codec::BodyCodec;
use super::{Data, NonZero};
use crate::error::RSocketResult;
use crate::frame::codec::chained;
use crate::frame::{codec, Flags, FrameHeader};

#[derive(Debug, Clone, From)]
pub struct Keepalive<'a> {
    pub last_received_position: NonZero<u64>,
    pub data: Data<'a>,
}

impl<'a> BodyCodec<'a> for Keepalive<'a> {
    fn decode(
        input: &'a [u8],
        _cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        chained(move |m| {
            Ok(Self {
                last_received_position: m.next()?,
                data: m.next()?,
            })
        })(input)
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
