use derive_more::{From, Into};

use super::fragment::Fragment;
use crate::frame::{Flags, Frame, FrameHeader};

#[derive(Debug, From, Into)]
pub(super) struct FramePartsError(Frame);

#[derive(Debug)]
pub(super) struct FrameParts<const MTU: usize> {
    header: FrameHeader,
    fragment: Fragment<MTU>,
    is_complete: bool,
}

impl<const MTU: usize> FrameParts<MTU> {
    #[inline]
    pub(super) fn new(frame: Frame) -> Result<Self, FramePartsError> {
        if !frame.header().flags().contains(Flags::FOLLOW) {
            return Err(frame.into());
        }

        let (header, variant) = frame.split();
        let fragment = match Fragment::<MTU>::new(variant) {
            | Ok(fragment) => fragment,
            | Err(err) => return Err(Frame::new(header, err.into()).into()),
        };

        Ok(Self {
            header,
            fragment,
            is_complete: false,
        })
    }

    pub(super) fn append(
        &mut self,
        frame: Frame,
    ) -> Result<bool, FramePartsError> {
        assert!(!self.is_complete, "frame is complete");

        let (header, variant) = frame.split();

        self.fragment.append(variant).map_err(move |err| {
            FramePartsError(Frame::new(header, err.into()))
        })?;

        if !header.flags().contains(Flags::FOLLOW) {
            self.is_complete = true;
        }

        Ok(self.is_complete)
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
