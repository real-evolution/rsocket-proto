use derive_more::Deref;
use recode::bytes;

use super::FrameHeader;

pub type Buffer = BufferWrapper<bytes::Bytes>;
pub type BufferMut = BufferWrapper<bytes::BytesMut>;

/// A wrapper type for buffers to add frame header context to using
/// encoders/ecoders.
#[derive(Debug, Clone, Deref)]
pub struct BufferWrapper<B> {
    #[deref]
    inner: B,
    header: FrameHeader,
}

impl<B> BufferWrapper<B> {
    /// Creates a new [`BufferWrapper<B>`] instance.
    ///
    /// # Parameters
    /// * `inner`: The inner buffer type to wrap.
    /// * `header`: Context header value.
    ///
    /// # Returns
    /// The created [`BufferWrapper<B>`] instance.
    pub fn new(inner: B, header: super::FrameHeader) -> Self {
        Self { inner, header }
    }

    /// Gets the inner header value.
    pub fn header(&self) -> &FrameHeader {
        &self.header
    }

    /// Splits [`self`] into a tuple of the wrapped buffer and the context
    /// header.
    pub fn into_parts(self) -> (B, FrameHeader) {
        (self.inner, self.header)
    }
}

impl bytes::Buf for Buffer {
    #[inline]
    fn remaining(&self) -> usize {
        self.inner.remaining()
    }

    #[inline]
    fn chunk(&self) -> &[u8] {
        self.inner.chunk()
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        self.inner.advance(cnt)
    }

    #[inline]
    fn copy_to_bytes(&mut self, len: usize) -> bytes::Bytes {
        self.inner.copy_to_bytes(len)
    }
}

unsafe impl bytes::BufMut for BufferMut {
    #[inline]
    fn remaining_mut(&self) -> usize {
        self.inner.remaining_mut()
    }

    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.inner.advance_mut(cnt)
    }

    #[inline]
    fn chunk_mut(&mut self) -> &mut bytes::buf::UninitSlice {
        self.inner.chunk_mut()
    }

    #[inline]
    fn put<T>(&mut self, src: T)
    where
        Self: Sized,
        T: bytes::Buf,
    {
        self.inner.put(src)
    }

    #[inline]
    fn put_slice(&mut self, src: &[u8]) {
        self.inner.put_slice(src)
    }

    #[inline]
    fn put_bytes(&mut self, val: u8, cnt: usize) {
        self.inner.put_bytes(val, cnt)
    }
}
