use std::io::Write;

use derive_more::From;

use super::codec::BodyCodec;
use super::util::chained;
use super::{Data, NonZero};
use crate::error::RSocketResult;
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone, From)]
pub struct Keepalive<'a> {
    pub last_received_position: NonZero<u64>,
    pub data: Data<'a>,
}

impl<'a> BodyCodec<'a> for Keepalive<'a> {
    fn decode(
        input: &'a [u8],
        _cx: &super::ParseContext,
    ) -> nom::IResult<&'a [u8], Self> {
        chained(move |m| {
            Ok(Self {
                last_received_position: m.next()?,
                data: m.next()?,
            })
        })(input)
    }

    fn encode<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.last_received_position.encode(writer)?;
        self.data.encode(writer)?;

        Ok(())
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::RESPOND)?
            .in_stream(0)?
            .done()
    }
}
