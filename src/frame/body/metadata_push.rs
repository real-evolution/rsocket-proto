use derive_more::From;
use nom::combinator::rest;

use super::codec::BodyCodec;
use crate::{
    error::RSocketResult,
    frame::{codec, Flags, FrameHeader},
};

#[derive(Debug, Clone, From)]
pub struct MetadataPush<'a> {
    pub metadata: &'a [u8],
}

impl<'a> BodyCodec<'a> for MetadataPush<'a> {
    fn decode(
        input: &'a [u8],
        _cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        codec::map_into(rest)(input)
    }

    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().flag_is(Flags::METADATA, true)?.done()
    }
}
