use super::codec::BodyCodec;
use super::util::{decode_chained, ChainedEncoder};
use super::{Data, NonZero};
use crate::error::RSocketResult;
use crate::frame::codec::{Decodable, Encodable};
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone)]
pub struct Keepalive<'a> {
    pub last_received_position: NonZero<u64>,
    pub data: Data<'a>,
}

impl<'a> Decodable<'a> for Keepalive<'a> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        decode_chained(move |m| {
            Ok(Self {
                last_received_position: m.next()?,
                data: m.next()?,
            })
        })(input)
    }
}

impl Encodable for Keepalive<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer
            .encode(&self.last_received_position)?
            .encode(&self.data)
    }
}

impl<'a> BodyCodec<'a> for Keepalive<'a> {
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::RESPOND)?
            .in_stream(0)?
            .done()
    }
}
