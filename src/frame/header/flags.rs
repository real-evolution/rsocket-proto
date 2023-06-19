bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Flags: u16 {
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

impl From<u16> for Flags {
    #[inline(always)]
    fn from(value: u16) -> Self {
        Self::from_bits_retain(value)
    }
}

impl From<Flags> for u16 {
    #[inline(always)]
    fn from(value: Flags) -> Self {
        value.bits()
    }
}
