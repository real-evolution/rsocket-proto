use recode::bytes::{self, Bytes};

use crate::frame::*;

pub(super) trait FragmentableVariant {
    const EXTRA_LEN: usize = 0;

    fn metadata(&mut self) -> Option<&mut bytes::Bytes>;
    fn data(&mut self) -> Option<&mut bytes::Bytes>;

    fn trim_to(&mut self, len: usize) -> (Bytes, Bytes) {
        let len = len - Self::EXTRA_LEN;

        let metadata = self
            .metadata()
            .map(|b| b.split_to(b.len().min(len)))
            .unwrap_or_default();

        let data = self
            .data()
            .map(|b| b.split_to(b.len().min(len - metadata.len())))
            .unwrap_or_default();

        (metadata, data)
    }
}

macro_rules! impl_for_req {
    ($variant:ty $(=> $extra_len:literal)?) => {
        impl FragmentableVariant for $variant {
            $(const EXTRA_LEN: usize = $extra_len;)?

            fn metadata(&mut self) -> Option<&mut bytes::Bytes> {
                self.metadata.as_mut().map(|b| b.as_mut())
            }

            fn data(&mut self) -> Option<&mut bytes::Bytes> {
                Some(self.data.as_inner_mut())
            }
        }
    };
}

impl_for_req!(RequestResponse);
impl_for_req!(RequestFNF);
impl_for_req!(RequestStream => 4);
impl_for_req!(RequestChannel => 4);

impl FragmentableVariant for Payload {
    fn metadata(&mut self) -> Option<&mut bytes::Bytes> {
        self.metadata.as_mut().map(|b| b.as_mut())
    }

    fn data(&mut self) -> Option<&mut bytes::Bytes> {
        self.data.as_mut().map(|b| b.as_inner_mut())
    }
}
