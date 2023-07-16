use derive_getters::{Dissolve, Getters};
use recode::bytes::{Bytes, BytesMut};
use recode::Recode;

use super::StreamId;

/// A tagged frame is a frame that has been tagged with a stream identifier.
///
/// The stream identifier is used to route the frame to the appropriate stream.
/// It is required for a frame to be tagged with a stream identifier when the
/// transport does not natively support multiplexing.
#[derive(Debug, Getters, Dissolve, Recode)]
#[dissolve(rename = "split")]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "Bytes"),
    encoder(buffer_type = "BytesMut")
)]
pub struct TaggedFrame {
    stream_id: StreamId,
    frame: super::Frame,
}

impl TaggedFrame {
    #[inline]
    pub(crate) const fn new(stream_id: StreamId, frame: super::Frame) -> Self {
        Self { stream_id, frame }
    }

    #[inline]
    pub(crate) fn frame_mut(&mut self) -> &mut super::Frame {
        &mut self.frame
    }
}
