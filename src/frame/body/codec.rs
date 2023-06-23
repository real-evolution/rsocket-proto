use std::io::Write;

use crate::error::RSocketResult;
use crate::frame::FrameHeader;

pub(crate) trait BodyCodec<'a>: Sized {
    fn encode<W: Write>(&self, writer: &mut W) -> std::io::Result<()>;

    #[inline(always)]
    fn validate_header(_header: &FrameHeader) -> RSocketResult<()> {
        Ok(())
    }
}

pub(super) trait EmptyBody: Sized {
    #[inline(always)]
    fn validate_header(_header: &FrameHeader) -> RSocketResult<()> {
        Ok(())
    }
}

impl<'a, T> BodyCodec<'a> for T
where
    T: EmptyBody + Default,
{
    #[inline(always)]
    fn encode<W: Write>(&self, _writer: &mut W) -> std::io::Result<()> {
        Ok(())
    }

    #[inline(always)]
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        <Self as EmptyBody>::validate_header(header)
    }
}
