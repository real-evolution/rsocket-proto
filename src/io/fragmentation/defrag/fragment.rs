use either::Either::{self, Left, Right};
use recode::bytes::{BufMut, Bytes, BytesMut};

use crate::frame::*;
use crate::io::fragmentation::variant::FragmentableVariant;

#[derive(Debug)]
pub(super) struct Fragment<const MTU: usize> {
    variant: FrameVariant,
    metadata: BytesMut,
    data: BytesMut,
}

impl<const MTU: usize> Fragment<MTU> {
    pub(super) fn new(variant: FrameVariant) -> Either<Self, FrameVariant> {
        match variant {
            | FrameVariant::RequestResponse(v) => Self::new_inner(v),
            | FrameVariant::RequestFNF(v) => Self::new_inner(v),
            | FrameVariant::RequestStream(v) => Self::new_inner(v),
            | FrameVariant::RequestChannel(v) => Self::new_inner(v),
            | FrameVariant::Payload(v) => Self::new_inner(v),
            | _ => Right(variant),
        }
    }

    #[inline]
    pub(super) fn append(
        &mut self,
        variant: FrameVariant,
    ) -> Option<FrameVariant> {
        let mut payload = match variant {
            | FrameVariant::Payload(p) => p,
            | _ => return Some(variant),
        };

        if let Some(mut metadata) = payload.metadata.take() {
            self.metadata.put(metadata.as_mut())
        }

        if let Some(mut data) = payload.data.take() {
            self.metadata.put(data.as_inner_mut())
        }

        None
    }

    #[inline]
    fn new_inner<V>(mut variant: V) -> Either<Self, FrameVariant>
    where
        V: FragmentableVariant + Into<FrameVariant>,
    {
        if variant.adjusted_len() <= MTU - FrameHeader::SIZE {
            return Right(variant.into());
        }

        Left(Self {
            metadata: Self::take_mut(variant.metadata()),
            data: Self::take_mut(variant.data()),
            variant: variant.into(),
        })
    }

    #[inline]
    fn take_mut(buf: Option<&mut Bytes>) -> BytesMut {
        if let Some(buf) = buf {
            if !buf.is_empty() {
                let mut ret = BytesMut::with_capacity(buf.len());
                ret.put(buf);
                return ret;
            }
        }

        BytesMut::new()
    }
}

impl<const MTU: usize> From<Fragment<MTU>> for FrameVariant {
    fn from(value: Fragment<MTU>) -> Self {
        let metadata = Some(value.metadata.freeze().into());
        let data = value.data.freeze().into();
        let variant = value.variant;

        match variant {
            | FrameVariant::RequestResponse(mut v) => {
                v.metadata = metadata;
                v.data = data;
                v.into()
            }
            | FrameVariant::RequestFNF(mut v) => {
                v.metadata = metadata;
                v.data = data;
                v.into()
            }
            | FrameVariant::RequestStream(mut v) => {
                v.metadata = metadata;
                v.data = data;
                v.into()
            }
            | FrameVariant::RequestChannel(mut v) => {
                v.metadata = metadata;
                v.data = data;
                v.into()
            }
            | FrameVariant::Payload(mut v) => {
                v.metadata = metadata;
                v.data = if data.is_empty() { None } else { Some(data) };
                v.into()
            }
            | _ => unreachable!("unexpected frame variant: {variant:?}"),
        }
    }
}
