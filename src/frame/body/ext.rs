use derive_more::From;
use nom::{combinator::rest, number::complete::be_u32, sequence::tuple};

use super::codec::BodyCodec;
use crate::{
    error::RSocketResult,
    frame::{codec, Flags, FrameHeader},
};

#[derive(Debug, Clone, From)]
pub struct Ext<'a> {
    pub extended_type: u32,
    pub metadata: Option<&'a [u8]>,
    pub data: Option<&'a [u8]>,
}

impl<'a> BodyCodec<'a> for Ext<'a> {
    fn decode(
        input: &'a [u8],
        cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        codec::map_into(tuple((
            be_u32,
            codec::length_metadata(cx),
            codec::none_if_empty(rest),
        )))(input)
    }

    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::IGNORE | Flags::METADATA)?
            .done()
    }
}
