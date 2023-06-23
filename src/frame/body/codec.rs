use crate::error::RSocketResult;
use crate::frame::FrameHeader;

pub(crate) trait BodyCodec<'a>: Sized {
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
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        <Self as EmptyBody>::validate_header(header)
    }
}
