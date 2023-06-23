use super::{codec::BodyCodec, NonZero};
use crate::error::RSocketResult;
use crate::frame::codec::Decodable;
use crate::frame::FrameHeader;

#[derive(Debug, Clone)]
pub struct RequestN {
    pub request_n: NonZero<u32>,
}

impl<'a> Decodable<'a> for RequestN {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        let (rem, request_n) = Decodable::decode(input)?;

        Ok((rem, Self { request_n }))
    }
}

impl<'a> BodyCodec<'a> for RequestN {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.request_n.encode(writer)?;

        Ok(())
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().has_empty_flags()?.done()
    }
}
