mod fragment;
mod parts;

use dashmap::{mapref::one::RefMut, DashMap};
use either::Either::{Left, Right};

use crate::frame::{Frame, StreamId};
use parts::FrameParts;

#[derive(Debug, Default)]
pub struct Defragmenter<const MTU: usize> {
    fragments: DashMap<StreamId, FrameParts<MTU>>,
}

impl<const MTU: usize> Defragmenter<MTU> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn defragment(&self, frame: Frame) -> crate::Result<Option<Frame>> {
        self.defragment_inner(frame)
    }

    #[inline]
    fn defragment_inner(&self, frame: Frame) -> crate::Result<Option<Frame>> {
        match self.fragments.get_mut(&frame.header().stream_id()) {
            | Some(parts) => self.append_frame(parts, frame),
            | None => self.insert_frame(frame),
        }
    }

    #[inline]
    fn append_frame(
        &self,
        parts: RefMut<'_, StreamId, FrameParts<MTU>>,
        frame: Frame,
    ) -> crate::Result<Option<Frame>> {
        let mut parts = parts;
        let stream_id = frame.header().stream_id();

        let is_complete = match parts.append(frame) {
            | Left(is_complete) => is_complete,
            | Right(frame) => return Ok(Some(frame)),
        };

        if is_complete {
            // drop map reference
            drop(parts);

            if let Some(entry) = self.fragments.remove(&stream_id) {
                return Ok(Some(entry.1.into()));
            }
        }

        Ok(None)
    }

    #[inline]
    fn insert_frame(&self, frame: Frame) -> crate::Result<Option<Frame>> {
        let parts = match FrameParts::<MTU>::new(frame) {
            | Left(parts) => parts,
            | Right(frame) => return Ok(Some(frame)),
        };

        match self.fragments.insert(parts.header().stream_id(), parts) {
            | Some(existing) => Err(crate::Error::UnexpectedEndOfFrame {
                stream_id: existing.header().stream_id(),
                frame_type: existing.header().frame_type(),
                message: "new frame started before previous was finished",
            }),
            | None => Ok(None),
        }
    }
}
