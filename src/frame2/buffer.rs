use std::ops::Deref;

use super::FrameHeader;

pub type Buffer = BufferWrapper<recode::bytes::Bytes>;
pub type BufferMut = BufferWrapper<recode::bytes::BytesMut>;

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

impl<B> Deref for BufferWrapper<B> {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
