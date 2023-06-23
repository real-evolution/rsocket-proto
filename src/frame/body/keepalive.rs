use super::util::{decode_chained, ChainedEncoder};
use crate::frame::codec::{Decodable, Encodable};

#[derive(Debug, Clone)]
pub struct Keepalive<'a> {
    pub last_received_position: super::NonZero<u64>,
    pub data: super::Data<'a>,
}

impl super::BodySpec for Keepalive<'_> {
    const FLAGS_MASK: crate::frame::Flags = crate::const_flags![RESPOND];
    const IS_CONNECTION_STREAM: bool = true;
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
