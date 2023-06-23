use super::{ChainedEncoder, Decodable, Encodable};

#[derive(Debug, Clone)]
pub struct ResumeOk {
    pub last_received_client_position: super::Number<u64>,
}

impl super::BodySpec for ResumeOk {
    const FLAGS_MASK: crate::frame::Flags = crate::const_flags![];
    const IS_CONNECTION_STREAM: bool = true;
}

impl<'a> Decodable<'a> for ResumeOk {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        let (rem, last_received_client_position) = Decodable::decode(input)?;

        Ok((
            rem,
            Self {
                last_received_client_position,
            },
        ))
    }
}

impl Encodable for ResumeOk {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer.encode(&self.last_received_client_position)
    }
}
