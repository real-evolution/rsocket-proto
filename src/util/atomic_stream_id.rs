use std::sync::atomic::{AtomicU32, Ordering};

use crate::frame::StreamId;

/// An atomic stream ID generator.
///
/// This is a simple atomic counter that generates stream IDs.
/// It can be used to generate sequenetial (with a step of `STEP`) stream IDs
/// for both client and server.
#[derive(Debug)]
pub struct AtomicStreamId<const STEP: u32 = 2> {
    current: AtomicU32,
}

impl<const STEP: u32> AtomicStreamId<STEP> {
    /// Creates a new atomic stream ID generator.
    ///
    /// # Parameters
    /// * `start` - The starting value for the stream ID.
    #[inline]
    pub const fn new(start: u32) -> Self {
        let current = AtomicU32::new(start);

        Self { current }
    }

    /// Creates a new atomic even stream ID generator.
    ///
    /// This is equivalent to calling [`Self::new(0)`].
    #[inline]
    pub const fn new_even() -> Self {
        Self::new(0)
    }

    /// Creates a new atomic odd stream ID generator.
    ///
    /// This is equivalent to calling [`Self::new(1)`].
    #[inline]
    pub const fn new_odd() -> Self {
        Self::new(1)
    }

    /// Gets the next stream ID in the sequence.
    #[inline]
    pub fn next(&self) -> StreamId {
        let value = self.current.fetch_add(STEP, Ordering::Relaxed);

        StreamId::new(value)
    }

    /// Gets the next stream ID in the sequence without incrementing the
    /// counter.
    #[inline]
    pub fn peek_next(&self) -> StreamId {
        let value = self.current.load(Ordering::Relaxed);

        StreamId::new(value + STEP)
    }

    /// Gets the current stream ID in the sequence.
    #[inline]
    pub fn current(&self) -> StreamId {
        let current = self.current.load(Ordering::Relaxed);

        StreamId::new(current)
    }
}
