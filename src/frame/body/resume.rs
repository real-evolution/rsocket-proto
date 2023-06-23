use derive_more::From;

use super::util::{decode_chained, ChainedEncoder};
use super::{codec::BodyCodec, Number, ResumeToken, Version};
use crate::error::RSocketResult;
use crate::frame::codec::{Decodable, Encodable};
use crate::frame::FrameHeader;

#[derive(Debug, Clone, From)]
pub struct Resume<'a> {
    pub version: Version,
    pub resume_identification_token: ResumeToken<'a>,
    pub last_received_server_position: Number<u64>,
    pub first_available_client_position: Number<u64>,
}

impl<'a> Decodable<'a> for Resume<'a> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        decode_chained(move |m| {
            Ok(Self {
                version: m.next()?,
                resume_identification_token: m.next()?,
                last_received_server_position: m.next()?,
                first_available_client_position: m.next()?,
            })
        })(input)
    }
}

impl Encodable for Resume<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer
            .encode(&self.version)?
            .encode(&self.resume_identification_token)?
            .encode(&self.last_received_server_position)?
            .encode(&self.first_available_client_position)
    }
}

impl<'a> BodyCodec<'a> for Resume<'a> {
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().has_empty_flags()?.in_stream(0)?.done()
    }
}
