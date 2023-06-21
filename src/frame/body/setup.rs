use std::io::Write;

use derive_more::From;
use nom::combinator::{cond, into, rest};
use nom::multi::length_data;
use nom::number::complete::{be_u16, be_u8};
use nom::sequence::tuple;

use super::codec::BodyCodec;
use super::version::Version;
use crate::error::RSocketResult;
use crate::frame::codec;
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone, From)]
pub struct Setup<'a> {
    pub version: Version,
    pub keepalive: u32,
    pub lifetime: u32,
    pub token: Option<&'a [u8]>,
    pub mime_metadata: &'a str,
    pub mime_data: &'a str,
    pub metadata: Option<&'a [u8]>,
    pub data: Option<&'a [u8]>,
}

impl<'a> BodyCodec<'a> for Setup<'a> {
    fn decode(
        input: &'a [u8],
        cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        into(tuple((
            // version
            Version::parse,
            // keepalive
            codec::non_zero_be_u32,
            // lifetime
            codec::non_zero_be_u32,
            // token
            cond(cx.header.flags.contains(Flags::RESUME), length_data(be_u16)),
            // mime_metadata
            codec::length_ascii(be_u8),
            // mime_data
            codec::length_ascii(be_u8),
            // metadata
            codec::length_metadata(cx),
            // data
            codec::none_if_empty(rest),
        )))(input)
    }

    fn encode<W: Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        todo!()
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::METADATA | Flags::RESUME | Flags::LEASE)?
            .in_stream(0)?
            .done()
    }
}
