use derive_getters::Getters;
use recode::bytes::Buf;
use recode::util::EncoderExt;
use recode::{Decoder, Encoder};

use crate::const_flags;
use crate::frame::Flags;

#[derive(Debug, Clone, Getters)]
pub struct Payload {
    pub(crate) metadata: Option<super::Metadata>,
    pub(crate) data: Option<super::Data>,
}

impl Decoder<super::Buffer> for Payload {
    type Error = crate::Error;

    fn decode(buf: &mut super::Buffer) -> Result<Self, Self::Error> {
        if buf
            .context()
            .flags()
            .contains(Flags::COMPLETE | Flags::NEXT)
        {
            return Err(Self::Error::UnexpectedFlag {
                flag: const_flags![COMPLETE | NEXT],
                frame_type: buf.context().frame_type(),
                message: "frame cannot have both COMPLETE and NEXT flags set",
            });
        }

        let metadata = super::Metadata::decode(buf)?;
        let data = if buf.has_remaining() {
            if !buf.context().flags().contains(Flags::NEXT) {
                return Err(Self::Error::UnexpectedFlag {
                    flag: Flags::NEXT,
                    frame_type: buf.context().frame_type(),
                    message: "payloads without NEXT flag must be empty",
                });
            }

            Some(super::Data::decode(buf)?)
        } else {
            None
        };

        Ok(Self { metadata, data })
    }
}

impl Encoder<super::BufferMut> for Payload {
    type Error = crate::Error;

    fn encode(
        item: &Self,
        buf: &mut super::BufferMut,
    ) -> Result<(), Self::Error> {
        debug_assert!(
            !buf.context()
                .flags()
                .contains(Flags::COMPLETE | Flags::NEXT),
            "an attempt to encode payload with both COMPLETE and NEXT flags",
        );

        debug_assert!(
            item.data.is_some() ^ !buf.context().flags().contains(Flags::NEXT),
            "an attempt to encode payload with NEXT flag but without data",
        );

        super::Metadata::encode(&item.metadata, buf)?;

        if let Some(data) = &item.data {
            data.encode_to(buf)?;
        }

        Ok(())
    }

    fn size_of(item: &Self, buf: &super::BufferMut) -> usize {
        let metadata_len = super::Metadata::size_of(&item.metadata, buf);
        let data_len =
            item.data.as_ref().map(|b| b.size(buf)).unwrap_or_default();

        metadata_len + data_len
    }
}
