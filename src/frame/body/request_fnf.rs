use derive_more::From;
use nom::{
    combinator::{into, rest},
    sequence::tuple,
};

use super::{codec::BodyCodec, util::metadata_opt};
use crate::frame::FrameHeader;

#[derive(Debug, Clone, From)]
pub struct RequestFNF<'a> {
    pub metadata: Option<&'a [u8]>,
    pub data: &'a [u8],
}

impl<'a> BodyCodec<'a> for RequestFNF<'a> {
    fn decode(
        header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        into(tuple((metadata_opt(header), rest)))(input)
    }

    fn encode<W: std::io::Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        todo!()
    }
}