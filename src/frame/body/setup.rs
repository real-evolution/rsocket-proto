use std::io::Write;

use derive_more::From;
use nom::combinator::{cond, into, verify};
use nom::multi::length_data;
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::sequence::tuple;

use super::parse::{BodyCodec, Parsable};
use super::util::{length_utf8, metadata_opt, rest_opt};
use super::version::Version;
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
        header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        into(tuple((
            // version
            Version::parse,
            // keepalive
            verify(be_u32, |&v| v > 0),
            // lifetime
            verify(be_u32, |&v| v > 0),
            // token
            cond(header.flags.contains(Flags::RESUME), length_data(be_u16)),
            // mime_metadata
            length_utf8(be_u8),
            // mime_data
            length_utf8(be_u8),
            // metadata
            metadata_opt(header),
            // data
            rest_opt,
        )))(input)
    }

    fn encode<W: Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        todo!()
    }
}
