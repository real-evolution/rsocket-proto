mod body;
mod codec;
mod header;
mod primitives;

pub use body::*;
pub use header::*;
pub use primitives::*;

use crate::error::{RSocketError, RSocketResult};

#[derive(Debug)]
pub struct Frame<'a> {
    pub(super) header: FrameHeader,
    pub(super) body: FrameBody<'a>,
    pub(super) raw: &'a [u8],
}

impl<'a> Frame<'a> {
    #[inline(always)]
    pub fn stream_id(&self) -> u32 {
        self.header.stream_id
    }

    #[inline(always)]
    pub fn frame_type(&self) -> FrameType {
        self.header.frame_type
    }

    #[inline(always)]
    pub fn flags(&self) -> Flags {
        self.header.flags
    }

    #[inline(always)]
    pub fn body(&self) -> &FrameBody<'a> {
        &self.body
    }

    #[inline(always)]
    pub fn raw(&self) -> &'a [u8] {
        self.raw
    }

    pub fn decode(input: &'a [u8]) -> RSocketResult<Self> {
        let (rem, header) = FrameHeader::decode(input)?;
        let cx = codec::ParseContext { header, raw: input };
        let (rem, body) = match header.frame_type {
            | FrameType::Setup => Self::decode_body::<Setup>(&cx, rem),
            | FrameType::Lease => Self::decode_body::<Lease>(&cx, rem),
            | FrameType::Keepalive => Self::decode_body::<Keepalive>(&cx, rem),
            | FrameType::RequestResponse => {
                Self::decode_body::<RequestResponse>(&cx, rem)
            }
            | FrameType::RequestFNF => {
                Self::decode_body::<RequestFNF>(&cx, rem)
            }
            | FrameType::RequestStream => {
                Self::decode_body::<RequestStream>(&cx, rem)
            }
            | FrameType::RequestChannel => {
                Self::decode_body::<RequestChannel>(&cx, rem)
            }
            | FrameType::RequestN => Self::decode_body::<RequestN>(&cx, rem),
            | FrameType::Cancel => Self::decode_body::<Cancel>(&cx, rem),
            | FrameType::Payload => Self::decode_body::<Payload>(&cx, rem),
            | FrameType::Error => Self::decode_body::<Error>(&cx, rem),
            | FrameType::MetadataPush => {
                Self::decode_body::<MetadataPush>(&cx, rem)
            }
            | FrameType::Resume => Self::decode_body::<Resume>(&cx, rem),
            | FrameType::ResumeOk => Self::decode_body::<ResumeOk>(&cx, rem),
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
        cx: &codec::ParseContext<'a>,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], FrameBody<'a>> {
        C::decode(input, cx).map(|(rest, body)| (rest, body.into()))
    }
}
