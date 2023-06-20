use derive_more::From;
use nom::{combinator::into, number::complete::be_u32, sequence::tuple};

use super::{
    codec::BodyCodec,
    util::{metadata_opt, rest_opt},
};
use crate::{
    error::RSocketResult,
    frame::{Flags, FrameHeader},
};

#[derive(Debug, Clone, From)]
pub struct Ext<'a> {
    pub extended_type: u32,
    pub metadata: Option<&'a [u8]>,
    pub data: Option<&'a [u8]>,
}

impl<'a> BodyCodec<'a> for Ext<'a> {
    fn decode(
        header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        into(tuple((be_u32, metadata_opt(header), rest_opt)))(input)
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
