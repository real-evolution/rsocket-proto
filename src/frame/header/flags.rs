bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RSocketFlags: u16 {
        const NEXT = 1 << 5;
        const COMPLETE = 1 << 6;
        const FOLLOW = 1 << 7;
        const METADATA = 1 << 8;
        const IGNORE = 1 << 9;
        const LEASE = Self::COMPLETE.bits();
        const RESUME = Self::FOLLOW.bits();
        const RESPOND = Self::FOLLOW.bits();
    }
}

impl From<u16> for RSocketFlags {
    fn from(value: u16) -> Self {
        Self::from_bits_retain(value)
    }
}

impl From<RSocketFlags> for u16 {
    fn from(value: RSocketFlags) -> Self {
        value.bits()
    }
}
