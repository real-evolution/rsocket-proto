use recode::bytes::{self, Bytes};

use crate::frame::*;

pub(super) trait FragmentableVariant: Into<FrameVariant> {
    const EXTRA_LEN: usize = 0;

    fn metadata(&mut self) -> Option<&mut bytes::Bytes>;
    fn data(&mut self) -> Option<&mut bytes::Bytes>;

    fn trim_to(&mut self, len: usize) -> (Bytes, Bytes) {
        let len = len - Self::EXTRA_LEN;

        let metadata = self
            .metadata()
            .map(|b| b.split_off(b.len().min(len)))
            .unwrap_or_default();

        let data = self
            .data()
            .map(|b| b.split_off(b.len().min(len - metadata.len())))
            .unwrap_or_default();

        (metadata, data)
    }

    #[inline]
    fn metadata_len(&mut self) -> usize {
        self.metadata()
            .as_ref()
            .map(|b| b.as_ref().len())
            .unwrap_or_default()
    }

    #[inline]
    fn data_len(&mut self) -> usize {
        self.data()
            .as_ref()
            .map(|b| b.as_ref().len())
            .unwrap_or_default()
    }

    #[inline]
    fn adjusted_len(&mut self) -> usize {
        self.metadata_len() + self.data_len() + Self::EXTRA_LEN
    }
}

macro_rules! impl_for_req {
    ($variant:ty $(=> $extra_len:literal)?) => {
        impl FragmentableVariant for $variant {
            $(const EXTRA_LEN: usize = $extra_len;)?

            #[inline]
            fn metadata(&mut self) -> Option<&mut bytes::Bytes> {
                self.metadata.as_mut().map(|b| b.as_mut())
            }

            #[inline]
            fn data(&mut self) -> Option<&mut bytes::Bytes> {
                Some(self.data.as_inner_mut())
            }

            #[inline]
            fn data_len(&mut self) -> usize {
                self.data.len()
            }
        }
    };
}

impl_for_req!(RequestResponse);
impl_for_req!(RequestFNF);
impl_for_req!(RequestStream => 4);
impl_for_req!(RequestChannel => 4);

impl FragmentableVariant for Payload {
    #[inline]
    fn metadata(&mut self) -> Option<&mut bytes::Bytes> {
        self.metadata.as_mut().map(|b| b.as_mut())
    }

    #[inline]
    fn data(&mut self) -> Option<&mut bytes::Bytes> {
        self.data.as_mut().map(|b| b.as_inner_mut())
    }
}
