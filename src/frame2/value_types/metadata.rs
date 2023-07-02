use std::ops::Deref;

use recode::bytes::{Buf, Bytes};
use recode::util::EncoderExt;
use recode::{codec, Decoder, Encoder};

use crate::frame2::{Buffer, BufferMut, Flags, FrameType};

type UnprefixedBuffer = codec::Buffer<recode::util::Remaining>;
type PrefixedBuffer = codec::Buffer<codec::u24>;

#[derive(Debug, Clone, Default)]
pub struct Metadata(Bytes);

impl Decoder<Buffer> for Metadata {
    type Error = crate::Error;

    fn decode(buf: &mut Buffer) -> Result<Metadata, Self::Error> {
        debug_assert!(buf.header().flags().contains(Flags::METADATA));

        if !buf.header().frame_type().supports_metadata() {
            return Err(Self::Error::ProtocolViolation(
                "frame does not support metadata",
            ));
        }

        let inner = match buf.header().frame_type() {
            | FrameType::MetadataPush => buf.copy_to_bytes(buf.remaining()),
            | _ => PrefixedBuffer::decode(buf)?.deref().clone(),
        };

        Ok(Self(inner))
    }
}

impl Decoder<Buffer> for Option<Metadata> {
    type Error = crate::Error;

    fn decode(buf: &mut Buffer) -> Result<Option<Metadata>, Self::Error> {
        if !buf.header().flags().contains(Flags::METADATA) {
            return if buf.header().frame_type().requires_metadata() {
                Err(Self::Error::ProtocolViolation(
                    "METADATA_PUSH frame type requires metadata",
                ))
            } else {
                Ok(None)
            };
        }

        Some(Metadata::decode(buf)).transpose()
    }
}

impl Encoder<BufferMut> for Metadata {
    type Error = crate::Error;

    fn encode(item: &Metadata, buf: &mut BufferMut) -> Result<(), Self::Error> {
        debug_assert!(buf.header().flags().contains(Flags::METADATA));
        debug_assert!(buf.header().frame_type().supports_metadata());

        let inner = item.0.clone();

        match buf.header().frame_type() {
            | FrameType::MetadataPush => {
                UnprefixedBuffer::new(inner).encode_to(buf)
            }
            | _ => PrefixedBuffer::new(inner).encode_to(buf),
        }
        .map_err(Into::into)
    }
}

impl Encoder<BufferMut> for Option<Metadata> {
    type Error = crate::Error;

    fn encode(
        item: &Option<Metadata>,
        buf: &mut BufferMut,
    ) -> Result<(), Self::Error> {
        if buf.header().flags().contains(Flags::METADATA) ^ item.is_some() {
            return Err(Self::Error::ProtocolViolation(
                "incosistent METADATA flag",
            ));
        }

        let Some(ref item) = item else {
            if buf.header().frame_type().requires_metadata() {
                return Err(Self::Error::ProtocolViolation(
                    "METADATA_PUSH frame type requires metadata",
                ));
            }

            return Ok(());
        };

        item.encode_to(buf)
    }
}

impl Deref for Metadata {
    type Target = Bytes;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FrameType {
    #[inline]
    pub const fn supports_metadata(&self) -> bool {
        match self {
            | FrameType::Error
            | FrameType::Keepalive
            | FrameType::RequestN
            | FrameType::Cancel => false,
            | _ => true,
        }
    }

    #[inline]
    pub const fn requires_metadata(&self) -> bool {
        matches!(self, FrameType::MetadataPush)
    }
}
