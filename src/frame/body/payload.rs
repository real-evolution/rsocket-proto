use derive_more::From;
use nom::{
    combinator::{cond, rest},
    sequence::tuple,
};

use super::codec::BodyCodec;
use crate::{
    error::RSocketResult,
    frame::{codec, Flags, FrameHeader},
};

#[derive(Debug, Clone, From)]
pub struct Payload<'a> {
    pub metadata: Option<&'a [u8]>,
    pub data: Option<&'a [u8]>,
}

impl<'a> BodyCodec<'a> for Payload<'a> {
    fn decode(
        input: &'a [u8],
        cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        codec::map_into(tuple((
            codec::length_metadata(cx),
            cond(!cx.header.flags.contains(Flags::COMPLETE), rest),
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
            .flags_match_mask(
                Flags::METADATA | Flags::FOLLOW | Flags::COMPLETE | Flags::NEXT,
            )?
            .done()
    }
}
