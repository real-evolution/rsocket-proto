mod fragment;
mod parts;

use dashmap::mapref::one::RefMut;
use dashmap::DashMap;
use either::Either::{Left, Right};
use parts::FrameParts;

use crate::frame::{FrameType, StreamId, TaggedFrame};

#[derive(Debug, Default)]
pub struct Defragmenter<const MTU: usize> {
    fragments: DashMap<StreamId, FrameParts<MTU>>,
}

impl<const MTU: usize> Defragmenter<MTU> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn defragment(
        &self,
        tagged: TaggedFrame,
    ) -> crate::Result<Option<TaggedFrame>> {
        if let FrameType::Cancel = tagged.frame().header().frame_type() {
            if self.fragments.remove(tagged.stream_id()).is_none() {
                return Ok(Some(tagged));
            }
        }

        self.defragment_inner(tagged)
    }

    #[inline]
    fn defragment_inner(
        &self,
        tagged: TaggedFrame,
    ) -> crate::Result<Option<TaggedFrame>> {
        match self.fragments.get_mut(tagged.stream_id()) {
            | Some(parts) => self.append_frame(parts, tagged),
            | None => self.insert_frame(tagged),
        }
    }

    fn append_frame(
        &self,
        parts: RefMut<'_, StreamId, FrameParts<MTU>>,
        tagged: TaggedFrame,
    ) -> crate::Result<Option<TaggedFrame>> {
        let mut parts = parts;
        let (stream_id, frame) = tagged.split();

        let is_complete = match parts.append(frame) {
            | Left(is_complete) => is_complete,
            | Right(frame) => {
                return Ok(Some(TaggedFrame::new(stream_id, frame)))
            }
        };

        if is_complete {
            // drop map reference
            drop(parts);

            if let Some(entry) = self.fragments.remove(&stream_id) {
                return Ok(Some(TaggedFrame::new(stream_id, entry.1.into())));
            }
        }

        Ok(None)
    }

    fn insert_frame(
        &self,
        tagged: TaggedFrame,
    ) -> crate::Result<Option<TaggedFrame>> {
        let (stream_id, frame) = tagged.split();

        let parts = match FrameParts::<MTU>::new(frame) {
            | Left(parts) => parts,
            | Right(frame) => {
                return Ok(Some(TaggedFrame::new(stream_id, frame)))
            }
        };

        match self.fragments.insert(stream_id, parts) {
            | Some(existing) => Err(crate::Error::UnexpectedEndOfFrame {
                stream_id,
                frame_type: existing.header().frame_type(),
                message: "new frame started before previous was finished",
            }),
            | None => Ok(None),
        }
    }
}
