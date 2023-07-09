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
            return Err(Self::Error::MissingFlag {
                flag: Flags::METADATA,
                frame_type: buf.context().frame_type(),
                message: "METADATA flag is missing, but metadata is present",
            });
        }

        if !buf.context().frame_type().supports_metadata() {
            return Err(Self::Error::UnexpectedFlag {
                flag: Flags::METADATA,
                frame_type: buf.context().frame_type(),
                message: "metadata is not supported for this frame",
            });
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
                Err(Self::Error::MissingFlag {
                    flag: Flags::METADATA,
                    frame_type: buf.context().frame_type(),
                    message: "frame requires metadata",
                })
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

    #[inline]
    fn size_of(item: &Self, buf: &BufferMut) -> usize {
        let len = item.0.len();

        if let FrameType::MetadataPush = buf.context().frame_type() {
            return len;
        }

        len + 3
    }
}

impl Encoder<BufferMut, Option<Metadata>> for Metadata {
    type Error = crate::Error;

    fn encode(
        item: &Option<Metadata>,
        buf: &mut BufferMut,
    ) -> Result<(), Self::Error> {
        if buf.context().flags().contains(Flags::METADATA) ^ item.is_some() {
            return Err(Self::Error::UnexpectedFlag {
                flag: Flags::METADATA,
                frame_type: buf.context().frame_type(),
                message: "METADATA flag is inconsistent with metadata presence",
            });
        }

        let Some(ref item) = item else {
            if buf.context().frame_type().requires_metadata() {
                return Err(Self::Error::MissingFlag {
                    flag: Flags::METADATA,
                    frame_type: buf.context().frame_type(),
                    message: "frame requires metadata",
                })
            }

            return Ok(());
        };

        item.encode_to(buf)
    }

    #[inline]
    fn size_of(item: &Option<Metadata>, buf: &BufferMut) -> usize {
        if let Some(ref item) = item {
            return item.size(buf);
        }

        0
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
