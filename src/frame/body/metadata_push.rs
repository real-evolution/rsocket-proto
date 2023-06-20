use derive_more::From;
use nom::combinator::{map, rest};

use super::parse::BodyCodec;
use crate::frame::FrameHeader;

#[derive(Debug, Clone, From)]
pub struct MetadataPush<'a> {
    pub metadata: &'a [u8],
}

impl<'a> BodyCodec<'a> for MetadataPush<'a> {
    fn decode(
        _header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        map(rest, Into::into)(input)
    }

    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }
}
