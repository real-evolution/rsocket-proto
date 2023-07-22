use std::sync::atomic::{AtomicU32, Ordering};

use crate::frame::StreamId;

#[derive(Debug)]
pub struct AtomicStreamId<const STEP: u32 = 2> {
    current: AtomicU32,
}

impl<const STEP: u32> AtomicStreamId<STEP> {
    #[inline]
    pub const fn new(start: u32) -> Self {
        let current = AtomicU32::new(start);

        Self { current }
    }

    #[inline]
    pub const fn new_even() -> Self {
        Self::new(0)
    }

    #[inline]
    pub const fn new_odd() -> Self {
        Self::new(1)
    }

    #[inline]
    pub fn next(&self) -> StreamId {
        let value = self.current.fetch_add(STEP, Ordering::Relaxed);

        StreamId::new(value - STEP)
    }

    #[inline]
    pub fn at(&self, steps: u32) -> StreamId {
        let value = self.current.fetch_add(STEP * steps, Ordering::Relaxed);

        StreamId::new(value)
    }

    #[inline]
    pub fn current(&self) -> StreamId {
        let current = self.current.load(Ordering::Relaxed);

        StreamId::new(current)
    }
}
