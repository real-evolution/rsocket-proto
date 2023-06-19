bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RSocketFlag: u16 {
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

impl From<u16> for RSocketFlag {
    fn from(value: u16) -> Self {
        Self::from_bits_retain(value)
    }
}

impl From<RSocketFlag> for u16 {
    fn from(value: RSocketFlag) -> Self {
        value.bits()
    }
}
