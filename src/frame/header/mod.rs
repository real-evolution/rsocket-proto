mod flags;
mod types;

pub use flags::Flags;
pub use types::FrameType;

use nom::combinator::map;
use nom::number::complete::{be_u16, be_u32};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Clone, Copy)]
pub struct FrameHeader {
    pub stream_id: u32,
    pub frame_type: FrameType,
    pub flags: Flags,
}

impl FrameHeader {
    #[inline(always)]
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], FrameHeader> {
        map(tuple((be_u32, be_u16)), |(stream_id, rem)| Self {
            stream_id,
            frame_type: ((rem >> 10) as u8).into(),
            flags: (rem % 0x03FFu16).into(),
        })(input)
    }
}
