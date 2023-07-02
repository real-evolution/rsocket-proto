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
