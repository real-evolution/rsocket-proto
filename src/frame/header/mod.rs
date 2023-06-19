mod flags;
mod types;

pub use flags::RSocketFlags;
use nom::combinator::map;
pub use types::RSocketFrameType;

use nom::number::complete::{be_u16, be_u32};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Clone, Copy)]
pub struct RSocketFrameHeader {
    pub stream_id: u32,
    pub frame_type: RSocketFrameType,
    pub flags: RSocketFlags,
}

impl RSocketFrameHeader {
    #[inline(always)]
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], RSocketFrameHeader> {
        map(tuple((be_u32, be_u16)), |(stream_id, rem)| Self {
            stream_id,
            frame_type: ((rem >> 10) as u8).into(),
            flags: (rem % 0x03FFu16).into(),
        })(input)
    }
}
