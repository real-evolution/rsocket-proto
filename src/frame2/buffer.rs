use recode::bytes;
use std::ops::Deref;

use super::FrameHeader;

pub type Buffer = BufferWrapper<bytes::Bytes>;
pub type BufferMut = BufferWrapper<bytes::BytesMut>;

#[derive(Debug, Clone)]
pub struct BufferWrapper<B> {
    inner: B,
    header: FrameHeader,
}

impl<B> BufferWrapper<B> {
    pub fn new(inner: B, header: super::FrameHeader) -> Self {
        Self { inner, header }
    }

    pub fn header(&self) -> &FrameHeader {
        &self.header
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

impl<B> Deref for BufferWrapper<B> {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
