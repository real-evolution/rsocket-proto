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

impl Flags {
    pub fn matches_mask(&self, mask: Flags) -> bool {
        (*self & mask) == *self
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

#[macro_export]
macro_rules! const_flags {
    () => {
        $crate::frame::Flags::empty()
    };

    ($($t:tt)*) => {
        $crate::frame::Flags::from_bits_truncate($crate::flags_to_bits!($($t)*))
    };
}

#[macro_export]
macro_rules! flags_to_bits {
    ($x:ident) => {
        $crate::frame::Flags::$x.bits()
    };

    (($($t:tt)+)) => {
        $crate::flags_to_bits!($($t)+)
    };

    ($lhs:ident $op:tt $($rest:tt)+) => {
        $crate::frame::Flags::$lhs.bits() $op $crate::flags_to_bits!($($rest)+)
    };
}
