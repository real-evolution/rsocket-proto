use super::{ChainedEncoder, Decodable, Encodable};

#[derive(Debug, Clone)]
pub struct RequestN {
    pub request_n: super::NonZero<u32>,
}

impl super::BodySpec for RequestN {
    const FLAGS_MASK: crate::frame::Flags = crate::const_flags![];
}

impl<'a> Decodable<'a> for RequestN {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        let (rem, request_n) = Decodable::decode(input)?;

        Ok((rem, Self { request_n }))
    }
}

impl Encodable for RequestN {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer.encode(&self.request_n)
    }
}
