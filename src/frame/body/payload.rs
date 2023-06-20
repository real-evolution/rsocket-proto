use derive_more::From;
use nom::{
    combinator::{cond, into, rest},
    sequence::tuple,
};

use super::{parse::BodyCodec, util::metadata_opt};
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone, From)]
pub struct Payload<'a> {
    pub metadata: Option<&'a [u8]>,
    pub data: Option<&'a [u8]>,
}

impl<'a> BodyCodec<'a> for Payload<'a> {
    fn decode(
        header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        into(tuple((
            metadata_opt(header),
            cond(!header.flags.contains(Flags::COMPLETE), rest),
        )))(input)
    }

    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }
}
