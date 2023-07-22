use either::Either::{self, Left, Right};

use super::fragment::Fragment;
use crate::frame::{Flags, Frame, FrameHeader};

#[derive(Debug)]
pub(super) struct FrameParts<const MTU: usize> {
    header: FrameHeader,
    fragment: Fragment<MTU>,
    is_complete: bool,
}

impl<const MTU: usize> FrameParts<MTU> {
    #[inline]
    pub(super) fn new(frame: Frame) -> Either<Self, Frame> {
        if !frame.header().flags().contains(Flags::FOLLOW) {
            return Right(frame);
        }

        let (header, variant) = frame.split();
        let fragment = match Fragment::<MTU>::new(variant) {
            | Left(fragment) => fragment,
            | Right(variant) => return Right(Frame::new(header, variant)),
        };

        Left(Self {
            header,
            fragment,
            is_complete: false,
        })
    }

    pub(super) fn append(&mut self, frame: Frame) -> Either<bool, Frame> {
        assert!(!self.is_complete, "frame is complete");

        let (header, variant) = frame.split();

        if let Some(variant) = self.fragment.append(variant) {
            return Right(Frame::new(header, variant));
        }

        if !header.flags().contains(Flags::FOLLOW) {
            self.is_complete = true;
        }

        Left(self.is_complete)
    }

    #[inline]
    pub(super) const fn header(&self) -> &FrameHeader {
        &self.header
    }
}

impl<const MTU: usize> From<FrameParts<MTU>> for Frame {
    fn from(value: FrameParts<MTU>) -> Self {
        assert!(value.is_complete, "incomplete frame");

        Frame::new(value.header, value.fragment.into())
    }
}
