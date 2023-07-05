use derive_more::{AsMut, AsRef, From, Into};

use recode::bytes::{Buf, Bytes};
use recode::util::EncoderExt;
use recode::{codec, Decoder, Encoder};

use crate::frame::{Buffer, BufferMut, Flags, FrameType};

type UnprefixedBuffer = codec::Buffer<recode::util::Remaining>;
type PrefixedBuffer = codec::Buffer<codec::u24>;

/// A value type wrapper to represent frames metadata.
#[derive(Debug, Clone, Default, Into, From, AsRef, AsMut)]
pub struct Metadata(Bytes);

impl Decoder<Buffer> for Metadata {
    type Error = crate::Error;

    fn decode(buf: &mut Buffer) -> Result<Metadata, Self::Error> {
        if !buf.context().flags().contains(Flags::METADATA) {
            return Err(Self::Error::ProtocolViolation(
                "frame header does not contain metadata flag",
            ));
        }

        if !buf.context().frame_type().supports_metadata() {
            return Err(Self::Error::ProtocolViolation(
                "frame does not support metadata",
            ));
        }

        let inner = match buf.context().frame_type() {
            | FrameType::MetadataPush => buf.copy_to_bytes(buf.remaining()),
            | _ => PrefixedBuffer::decode(buf)?.into_inner(),
        };

        Ok(Self(inner))
    }
}

impl Decoder<Buffer, Option<Metadata>> for Metadata {
    type Error = crate::Error;

    fn decode(buf: &mut Buffer) -> Result<Option<Metadata>, Self::Error> {
        if !buf.context().flags().contains(Flags::METADATA) {
            return if buf.context().frame_type().requires_metadata() {
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
        debug_assert!(buf.context().flags().contains(Flags::METADATA));
        debug_assert!(buf.context().frame_type().supports_metadata());

        let inner = item.0.clone();

        match buf.context().frame_type() {
            | FrameType::MetadataPush => {
                UnprefixedBuffer::new(inner).encode_to(buf)
            }
            | _ => PrefixedBuffer::new(inner).encode_to(buf),
        }
        .map_err(Into::into)
    }
}

impl Encoder<BufferMut, Option<Metadata>> for Metadata {
    type Error = crate::Error;

    fn encode(
        item: &Option<Metadata>,
        buf: &mut BufferMut,
    ) -> Result<(), Self::Error> {
        if buf.context().flags().contains(Flags::METADATA) ^ item.is_some() {
            return Err(Self::Error::ProtocolViolation(
                "incosistent METADATA flag",
            ));
        }

        let Some(ref item) = item else {
            if buf.context().frame_type().requires_metadata() {
                return Err(Self::Error::ProtocolViolation(
                    "METADATA_PUSH frame type requires metadata",
                ));
            }

            return Ok(());
        };

        item.encode_to(buf)
    }
}

impl FrameType {
    /// Gets whether the frame type supports metadata or not.
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

    /// Gets whether the frame type requires metadata or not.
    #[inline]
    pub const fn requires_metadata(&self) -> bool {
        matches!(self, FrameType::MetadataPush)
    }
}
