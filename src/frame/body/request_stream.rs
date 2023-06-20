use derive_more::From;
use nom::{
    combinator::{into, rest, verify},
    number::complete::be_u32,
    sequence::tuple,
};

use super::{codec::BodyCodec, util::metadata_opt};
use crate::{frame::{FrameHeader, Flags}, error::RSocketResult};

#[derive(Debug, Clone, From)]
pub struct RequestStream<'a> {
    pub initial_request_n: u32,
    pub metadata: Option<&'a [u8]>,
    pub data: &'a [u8],
}

impl<'a> BodyCodec<'a> for RequestStream<'a> {
    fn decode(
        header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        into(tuple((
            verify(be_u32, |&v| v > 0),
            metadata_opt(header),
            rest,
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
            .flags_match_mask(Flags::METADATA | Flags::FOLLOW)?
            .done()
    }
}
