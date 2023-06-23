mod body;
mod codec;
mod header;
mod primitives;

pub use body::*;
pub use header::*;
pub use primitives::*;

use crate::error::{RSocketError, RSocketResult};

use self::codec::ContextDecodable;

#[derive(Debug)]
pub struct Frame<'a> {
    pub(super) header: FrameHeader,
    pub(super) body: FrameBody<'a>,
    pub(super) raw: &'a [u8],
}

impl<'a> Frame<'a> {
    #[inline(always)]
    pub fn stream_id(&self) -> u32 {
        self.header.stream_id
    }

    #[inline(always)]
    pub fn frame_type(&self) -> FrameType {
        self.header.frame_type
    }

    #[inline(always)]
    pub fn flags(&self) -> Flags {
        self.header.flags
    }

    #[inline(always)]
    pub fn body(&self) -> &FrameBody<'a> {
        &self.body
    }

    #[inline(always)]
    pub fn raw(&self) -> &'a [u8] {
        self.raw
    }

    pub fn decode(input: &'a [u8]) -> RSocketResult<Self> {
        let (rem, header) = FrameHeader::decode(input)?;
        let (rem, body) =
            FrameBody::decode_with(rem, &BodyDecodeContext { header })?;

        if !rem.is_empty() {
            return Err(RSocketError::BufferLength(
                "input buffer was left with remaining bytes",
            ));
        }

        Ok(Self {
            header,
            body,
            raw: input,
        })
    }
}
