use derive_more::From;

use super::{ChainedEncoder, Decodable, Encodable};

#[derive(Debug, Clone, From)]
pub struct Resume<'a> {
    pub version: super::Version,
    pub resume_identification_token: super::ResumeToken<'a>,
    pub last_received_server_position: super::Number<u64>,
    pub first_available_client_position: super::Number<u64>,
}

impl super::BodySpec for Resume<'_> {
    const FLAGS_MASK: crate::frame::Flags = crate::const_flags![];
    const IS_CONNECTION_STREAM: bool = true;
}

impl<'a> Decodable<'a> for Resume<'a> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        super::decode_chained(move |m| {
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
