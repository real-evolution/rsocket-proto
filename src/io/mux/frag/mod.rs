use recode::bytes::Bytes;

use crate::frame::{Frame, FrameVariant, StreamId, TaggedFrame};

#[derive(Debug)]
pub struct Fragmenter<const MTU: usize> {
    stream_id: StreamId,
    metadata: Bytes,
    data: Bytes,
}

impl<const MTU: usize> Fragmenter<MTU> {
    pub fn fragment(
        mut tagged: TaggedFrame,
    ) -> impl Iterator<Item = TaggedFrame> {
        let stream_id = *tagged.stream_id();
        let (metadata, data) = Self::trim_frame(&mut tagged);

        std::iter::once(tagged).chain(Self {
            stream_id,
            metadata,
            data,
        })
    }

    #[inline]
    fn trim_frame(tagged: &mut TaggedFrame) -> (Bytes, Bytes) {
        use super::variant::FragmentableVariant;

        match tagged.frame_mut().variant_mut() {
            | FrameVariant::RequestResponse(ref mut v) => v.trim_to(MTU),
            | FrameVariant::RequestFNF(ref mut v) => v.trim_to(MTU),
            | FrameVariant::RequestStream(ref mut v) => v.trim_to(MTU),
            | FrameVariant::RequestChannel(ref mut v) => v.trim_to(MTU),
            | FrameVariant::Payload(ref mut v) => v.trim_to(MTU),
            | _ => Default::default(),
        }
    }

    #[inline]
    const fn has_remaining(&self) -> bool {
        self.data.len() + self.metadata.len() > 0
    }

    #[inline]
    fn take_from(bytes: &mut Bytes, len: usize) -> (usize, Option<Bytes>) {
        match bytes.len().min(len) {
            | 0 => (0, None),
            | l => (l, Some(bytes.split_to(l))),
        }
    }
}

impl<const MTU: usize> Iterator for Fragmenter<MTU> {
    type Item = TaggedFrame;

    fn next(&mut self) -> Option<Self::Item> {
        let (len, metadata) = Self::take_from(&mut self.metadata, MTU);
        let (_, data) = Self::take_from(&mut self.metadata, len - MTU);

        if !self.has_remaining() {
            return None;
        }

        let builder = Frame::builder()
            .payload()
            .metadata(metadata.map(Into::into))
            .data(data.map(Into::into));

        if self.has_remaining() {
            return Some(builder.incomplete().unwrap().tagged(self.stream_id));
        }

        Some(builder.build().unwrap().tagged(self.stream_id))
    }
}
