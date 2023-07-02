use bitflags::bitflags;

bitflags! {
    /// A flags structure to represent possible fram flags.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Flags: u16 {
        /// (N)ext: bit to indicate Next (Payload Data and/or Metadata present).
        ///
        /// If set, onNext(Payload) or equivalent will be invoked on
        /// Subscriber/Observer.
        const NEXT = 1 << 5;

        /// (C)omplete: bit to indicate stream completion.
        ///
        /// If set, `onComplete()` or equivalent will be invoked on
        /// Subscriber/Observer.
        const COMPLETE = 1 << 6;

        /// (F)ollows: More fragments follow this fragment.
        const FOLLOW = 1 << 7;

        /// (M)etadata: Metadata present.
        const METADATA = 1 << 8;

        /// (I)gnore: Ignore invalid frames.
        const IGNORE = 1 << 9;

        /// (L)ease: Will honor LEASE (or not).
        const LEASE = Self::COMPLETE.bits();

        /// (R)esume Enable: Client requests resume capability if possible.
        /// Resume Identification Token present.
        const RESUME = Self::FOLLOW.bits();

        /// (R)espond with KEEPALIVE or not.
        const RESPOND = Self::FOLLOW.bits();
    }
}

/// Creates a value of type [`Flags`] at compile time.
///
/// This macro removes the boilerplate required to create constant [`Flags`]
/// values at compile time.
///
/// # Example
/// ```rust
///  struct MyType;
///
///  impl MyType {
///     const FLAGS_MASK: Flags = crate::const_flags![METADATA | RESUME | LEASE];
///  }
///
/// ```
#[macro_export]
macro_rules! const_flags {
    () => {
        $crate::frame2::Flags::empty()
    };

    ($($t:tt)*) => {
        $crate::frame2::Flags::from_bits_truncate($crate::flags_to_bits!($($t)*))
    };
}

/// Converts an expression that uses [`Flags`] constants into an expression
/// that uses the backing vaue of [`Flags`] ([`u16`]).
#[macro_export]
macro_rules! flags_to_bits {
    ($x:ident) => {
        $crate::frame2::Flags::$x.bits()
    };

    (($($t:tt)+)) => {
        $crate::flags_to_bits!($($t)+)
    };

    ($lhs:ident $op:tt $($rest:tt)+) => {
        $crate::frame2::Flags::$lhs.bits() $op $crate::flags_to_bits!($($rest)+)
    };
}
