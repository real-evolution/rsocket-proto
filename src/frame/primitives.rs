use derive_more::{Deref, From, Into};
use nom::{combinator::map, number::complete::be_u24, IResult};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, From, Into, Deref,
)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u24(u32);

impl u24 {
    pub const SIZE: usize = 3;
    pub const MIN: u32 = 0x0000_0000;
    pub const MAX: u32 = 0x00FF_FFFF;

    #[inline(always)]
    pub fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(be_u24, Into::into)(input)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_decode() {
        const VALID_TEST_INPUT: &[u8] = &[0x12, 0x34, 0x56, 0x78];
        const INVALID_TEST_INPUT: &[u8] = &[0x12, 0x34];

        let (rem, output) = super::u24::decode(VALID_TEST_INPUT).unwrap();

        assert_eq!(rem, &[0x78]);
        assert_eq!(output, 0x123456u32.into());

        assert_eq!(
            Err(nom::Err::Incomplete(nom::Needed::new(super::u24::SIZE))),
            super::u24::decode(INVALID_TEST_INPUT)
        );
    }
}
