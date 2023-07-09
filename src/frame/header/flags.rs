use bitflags::bitflags;
use derive_more::Display;

bitflags! {
    /// A flags structure to represent possible fram flags.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Display)]
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
    ($($expr:tt)*) => {
        $crate::process_flags_expr!(@entry: [ $($expr)* ] [] )
    };
}

/// Recursively Converts an expression that uses [`Flags`] constants into
/// an expression that uses the backing value of [`Flags`] ([`u16`]).
#[macro_export]
macro_rules! process_flags_expr {
    (@entry: [ ] [ ]) => {
        $crate::frame::Flags::empty()
    };

    (@entry: [ $($lhs:tt)+ ] [ ]) => {
        $crate::process_flags_expr! {
            @process:
                [ $($lhs)+ ]
                []
        }
    };

    (@process: [ $flag:ident $($lhs:tt)* ] [ $($rhs:tt)* ]) => {
        $crate::process_flags_expr! {
            @process:
                [ $($lhs)* ]
                [ $($rhs)* $crate::frame::Flags::$flag.bits() ]
        }
    };

    (@process: [ ( $($expr:tt)+ ) $($lhs:tt)* ] [ $($rhs:tt)* ]) => {
        $crate::process_flags_expr! {
            @process:
                [ $($lhs)* ]
                [ $($rhs)* $crate::const_flags!($($expr)+).bits() ]
        }
    };

    (@process: [ $other:tt $($lhs:tt)* ] [ $($rhs:tt)* ]) => {
        $crate::process_flags_expr! {
            @process:
                [ $($lhs)* ]
                [ $($rhs)* $other ]
        }
    };

    (@process: [ ] [ $($lhs:tt)+ ]) => {
        $crate::frame::Flags::from_bits_truncate($($lhs)+)
    };
}
