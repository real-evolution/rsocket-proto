use crate::error::{RSocketError, RSocketResult};

use super::{Flags, FrameHeader};

#[derive(Debug)]
pub(crate) struct FrameHeaderValidator<'a>(&'a FrameHeader);

impl<'a> FrameHeaderValidator<'a> {
    #[inline(always)]
    pub(crate) fn new(header: &'a FrameHeader) -> Self {
        Self(header)
    }

    #[inline(always)]
    pub(crate) fn flags_match_mask(self, mask: Flags) -> RSocketResult<Self> {
        if !self.0.flags.matches_mask(mask) {
            return Err(RSocketError::UnexpectedFlags {
                flags: self.0.flags,
                mask,
            });
        }

        Ok(self)
    }

    #[inline(always)]
    pub(crate) fn has_empty_flags(self) -> RSocketResult<Self> {
        self.flags_match_mask(Flags::empty())
    }

    #[inline(always)]
    pub(crate) fn flag_is(
        self,
        flag: Flags,
        expected_value: bool,
    ) -> RSocketResult<Self> {
        if !self.0.flags.contains(flag) {
            return Err(RSocketError::UnexpectedFlagValue {
                flag,
                expected_value,
            });
        }

        Ok(self)
    }

    #[inline(always)]
    pub(crate) fn in_stream(self, stream_id: u32) -> RSocketResult<Self> {
        if self.0.stream_id != 0 {
            return Err(RSocketError::UnexpectedStreamId {
                expected: stream_id,
                actual: self.0.stream_id,
            });
        }

        Ok(self)
    }

    #[inline(always)]
    pub(crate) fn done(self) -> RSocketResult<()> {
        Ok(())
    }
}
