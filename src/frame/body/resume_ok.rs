use derive_more::From;

use super::{codec::BodyCodec, Number};
use crate::error::RSocketResult;
use crate::frame::codec::Decodable;
use crate::frame::FrameHeader;

#[derive(Debug, Clone, From)]
pub struct ResumeOk {
    pub last_received_client_position: Number<u64>,
}

impl<'a> BodyCodec<'a> for ResumeOk {
    fn decode(
        input: &'a [u8],
        _cx: &super::ParseContext,
    ) -> nom::IResult<&'a [u8], Self> {
        let (rem, last_received_client_position) = Number::decode(input)?;

        Ok((
            rem,
            Self {
                last_received_client_position,
            },
        ))
    }

    fn encode<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.last_received_client_position.encode(writer)?;

        Ok(())
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().has_empty_flags()?.in_stream(0)?.done()
    }
}
