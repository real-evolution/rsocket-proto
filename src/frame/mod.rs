mod body;
mod header;
mod primitives;

pub use body::*;
pub use header::*;
pub use primitives::*;

use crate::error::{RSocketError, RSocketResult};

#[derive(Debug)]
pub struct Frame<'a> {
    pub header: FrameHeader,
    pub body: FrameBody<'a>,
    pub raw: &'a [u8],
}

impl<'a> Frame<'a> {
    pub fn decode(input: &'a [u8]) -> RSocketResult<Self> {
        let (rem, header) = FrameHeader::decode(input)?;
        let (rem, body) = match header.frame_type {
            | FrameType::Setup => Self::decode_body::<Setup>(&header, rem),
            | FrameType::Lease => Self::decode_body::<Lease>(&header, rem),
            | FrameType::Keepalive => {
                Self::decode_body::<Keepalive>(&header, rem)
            }
            | FrameType::RequestResponse => {
                Self::decode_body::<RequestResponse>(&header, rem)
            }
            | FrameType::RequestFNF => {
                Self::decode_body::<RequestFNF>(&header, rem)
            }
            | FrameType::RequestStream => {
                Self::decode_body::<RequestStream>(&header, rem)
            }
            | FrameType::RequestChannel => {
                Self::decode_body::<RequestChannel>(&header, rem)
            }
            | FrameType::RequestN => {
                Self::decode_body::<RequestN>(&header, rem)
            }
            | FrameType::Cancel => Self::decode_body::<Cancel>(&header, rem),
            | FrameType::Payload => Self::decode_body::<Payload>(&header, rem),
            | FrameType::Error => Self::decode_body::<Error>(&header, rem),
            | FrameType::MetadataPush => {
                Self::decode_body::<MetadataPush>(&header, rem)
            }
            | FrameType::Resume => Self::decode_body::<Resume>(&header, rem),
            | FrameType::ResumeOk => {
                Self::decode_body::<ResumeOk>(&header, rem)
            }
            | FrameType::Other(t) => {
                return Err(RSocketError::UnknownFrameType(t))
            }
        }?;

        if !rem.is_empty() {
            return Err(RSocketError::BufferLength(
                "input buffer was left with remaining bytes",
            ));
        }

        Ok(Self {
            header,
            body,
            raw: input,
        })
    }

    #[inline(always)]
    fn decode_body<C: BodyCodec<'a> + Into<FrameBody<'a>>>(
        header: &header::FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], FrameBody<'a>> {
        C::decode(header, input).map(|(rest, body)| (rest, body.into()))
    }
}
