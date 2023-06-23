use super::util::ChainedEncoder;
use crate::frame::codec::{Decodable, Encodable};

#[derive(Debug, Clone)]
pub struct MetadataPush<'a> {
    pub metadata: super::RestMetadata<'a>,
}

impl super::BodySpec for MetadataPush<'_> {
    const FLAGS_MASK: crate::frame::Flags = crate::const_flags![METADATA];
    const REQUIRED_FLAGS: crate::frame::Flags = crate::const_flags![METADATA];
}

impl<'a> Decodable<'a> for MetadataPush<'a> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        let (rem, metadata) = Decodable::decode(input)?;

        Ok((rem, Self { metadata }))
    }
}

impl Encodable for MetadataPush<'_> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        writer.encode(&self.metadata)
    }
}
