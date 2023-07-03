use recode::{bytes::Buf, util::EncoderExt, Decoder, Encoder};

use crate::frame::Flags;

#[derive(Debug, Clone)]
pub struct Payload {
    pub metadata: Option<super::Metadata>,
    pub data: Option<super::Data>,
}

impl Decoder<super::Buffer> for Payload {
    type Error = crate::Error;

    fn decode(buf: &mut super::Buffer) -> Result<Self, Self::Error> {
        if buf.header().flags().contains(Flags::COMPLETE | Flags::NEXT) {
            return Err(crate::Error::ProtocolViolation(
                "payloads cannot have both COMPLETE and NEXT flags set",
            ));
        }

        let metadata = Option::<super::Metadata>::decode(buf)?;
        let data = if buf.has_remaining() {
            if !buf.header().flags().contains(Flags::NEXT) {
                return Err(crate::Error::ProtocolViolation(
                    "payloads without NEXT flag must be empty",
                ));
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
            !buf.header().flags().contains(Flags::COMPLETE | Flags::NEXT),
            "an attempt to encode payload with both COMPLETE and NEXT flags",
        );

        debug_assert!(
            item.data.is_some() ^ !buf.header().flags().contains(Flags::NEXT),
            "an attempt to encode payload with NEXT flag but without data",
        );

        item.metadata.encode_to(buf)?;

        if let Some(data) = &item.data {
            data.encode_to(buf)?;
        }

        Ok(())
    }
}
