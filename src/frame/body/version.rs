use nom::number::complete::be_u32;

use crate::frame::codec;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}

impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version { major: 1, minor: 0 }
    }
}

impl From<u32> for Version {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self {
            major: (value >> 16) as u16,
            minor: (value & 0xffff) as u16,
        }
    }
}

impl From<Version> for u32 {
    #[inline(always)]
    fn from(value: Version) -> Self {
        ((value.major as u32) << 16) | (value.minor as u32)
    }
}

impl Version {
    pub(crate) fn parse<I, E>(input: I) -> nom::IResult<I, Version, E>
    where
        I: nom::Slice<std::ops::RangeFrom<usize>>
            + nom::InputIter<Item = u8>
            + nom::InputLength
            + nom::InputTake,
        E: nom::error::ParseError<I>,
    {
        codec::map_into(be_u32)(input)
    }
}
