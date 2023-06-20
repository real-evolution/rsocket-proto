use std::io::Write;

use crate::frame::FrameHeader;

pub(super) trait Parsable: Sized {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self>;
}

pub(super) trait BodyCodec<'a>: Sized {
    fn decode(
        header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self>;

    fn encode<W: Write>(&self, writer: &mut W) -> std::io::Result<()>;
}

pub(super) trait EmptyBody: Sized {}

impl<'a, T> BodyCodec<'a> for T
where
    T: EmptyBody + Default,
{
    #[inline(always)]
    fn decode(
        _header: &FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        Ok((input, Default::default()))
    }

    #[inline(always)]
    fn encode<W: Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        Ok(())
    }
}
