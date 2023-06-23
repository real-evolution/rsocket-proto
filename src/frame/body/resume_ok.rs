use super::{codec::BodyCodec, Number};
use crate::error::RSocketResult;
use crate::frame::codec::{Decodable, Encodable};
use crate::frame::FrameHeader;

#[derive(Debug, Clone)]
pub struct ResumeOk {
    pub last_received_client_position: Number<u64>,
}

impl<'a> Decodable<'a> for ResumeOk {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        let (rem, last_received_client_position) = Number::decode(input)?;

        Ok((
            rem,
            Self {
                last_received_client_position,
            },
        ))
    }
}

impl Encodable for ResumeOk {
    fn encode<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.last_received_client_position.encode(writer)?;

        Ok(())
    }
}

impl<'a> BodyCodec<'a> for ResumeOk {
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().has_empty_flags()?.in_stream(0)?.done()
    }
}
