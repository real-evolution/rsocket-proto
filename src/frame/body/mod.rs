mod cancel;
mod codec;
mod error;
mod ext;
mod keepalive;
mod lease;
mod metadata_push;
mod payload;
mod request_channel;
mod request_fnf;
mod request_n;
mod request_response;
mod request_stream;
mod resume;
mod resume_ok;
mod setup;
mod value_types;

pub use cancel::Cancel;
pub use error::Error;
pub use ext::Ext;
pub use keepalive::Keepalive;
pub use lease::Lease;
pub use metadata_push::MetadataPush;
pub use payload::Payload;
pub use request_channel::RequestChannel;
pub use request_fnf::RequestFNF;
pub use request_n::RequestN;
pub use request_response::RequestResponse;
pub use request_stream::RequestStream;
pub use resume::Resume;
pub use resume_ok::ResumeOk;
pub use setup::Setup;
pub use value_types::*;

pub(self) use codec::*;

use derive_more::From;

use crate::error::{RSocketError, RSocketResult};

#[derive(Debug, From)]
pub enum FrameBody<'a> {
    Setup(Setup<'a>),
    Error(Error<'a>),
    Lease(Lease<'a>),
    Keepalive(Keepalive<'a>),
    RequestResponse(RequestResponse<'a>),
    RequestFNF(RequestFNF<'a>),
    RequestStream(RequestStream<'a>),
    RequestChannel(RequestChannel<'a>),
    RequestN(RequestN),
    Cancel(Cancel),
    Payload(Payload<'a>),
    MetadataPush(MetadataPush<'a>),
    Ext(Ext<'a>),
    Resume(Resume<'a>),
    ResumeOk(ResumeOk),
}

pub(crate) trait BodySpec {
    const FLAGS_MASK: super::Flags;
    const REQUIRED_FLAGS: super::Flags = super::Flags::empty();
    const IS_CONNECTION_STREAM: bool = false;
}

#[derive(Debug)]
pub(crate) struct BodyDecodeContext {
    pub(crate) header: crate::frame::FrameHeader,
}

impl<'a> FrameBody<'a> {
    pub(crate) fn decode(
        input: &'a [u8],
        cx: &BodyDecodeContext,
    ) -> RSocketResult<Self> {
        use crate::frame::FrameType;

        match cx.header.frame_type {
            | FrameType::Setup => Self::take::<Setup>(input, cx),
            | FrameType::Lease => Self::take::<Lease>(input, cx),
            | FrameType::Keepalive => Self::take::<Keepalive>(input, cx),
            | FrameType::RequestResponse => {
                Self::take::<RequestResponse>(input, cx)
            }
            | FrameType::RequestFNF => Self::take::<RequestFNF>(input, cx),
            | FrameType::RequestStream => {
                Self::take::<RequestStream>(input, cx)
            }
            | FrameType::RequestChannel => {
                Self::take::<RequestChannel>(input, cx)
            }
            | FrameType::RequestN => Self::take::<RequestN>(input, cx),
            | FrameType::Cancel => Self::take::<Cancel>(input, cx),
            | FrameType::Payload => Self::take::<Payload>(input, cx),
            | FrameType::Error => Self::take::<Error>(input, cx),
            | FrameType::MetadataPush => Self::take::<MetadataPush>(input, cx),
            | FrameType::Resume => Self::take::<Resume>(input, cx),
            | FrameType::ResumeOk => Self::take::<ResumeOk>(input, cx),
            | FrameType::Other(t) => Err(RSocketError::UnknownFrameType(t)),
        }
    }
}

impl<'a, 'b> FrameBody<'a> {
    fn take<B>(
        input: &'a [u8],
        cx: &'b BodyDecodeContext,
    ) -> RSocketResult<Self>
    where
        B: ContextDecodable<'a, &'b BodyDecodeContext> + Into<Self> + BodySpec,
    {
        let (rem, body) = B::decode_with(input, cx)?;

        if !rem.is_empty() {
            return Err(RSocketError::BufferLength(
                "input buffer was left with remaining bytes",
            ));
        }

        Self::validate::<B>(cx)?;

        Ok(body.into())
    }

    #[inline(always)]
    fn validate<B>(cx: &BodyDecodeContext) -> RSocketResult<()>
    where
        B: BodySpec,
    {
        let crate::frame::FrameHeader {
            stream_id, flags, ..
        } = cx.header;

        if B::IS_CONNECTION_STREAM && stream_id != 0 {
            return Err(RSocketError::UnexpectedStreamId {
                expected: 0,
                actual: cx.header.stream_id,
            });
        }

        if !B::REQUIRED_FLAGS.is_empty()
            && !cx.header.flags.contains(B::REQUIRED_FLAGS)
        {
            return Err(RSocketError::UnexpectedFlagValue {
                flag: B::REQUIRED_FLAGS,
                expected_value: true,
            });
        }

        if cx.header.flags.contains(!B::FLAGS_MASK) {
            return Err(RSocketError::UnexpectedFlags {
                flags,
                mask: B::FLAGS_MASK,
            });
        }

        Ok(())
    }
}

impl<'a, B: Decodable<'a>> ContextDecodable<'a, &BodyDecodeContext> for B {
    #[inline(always)]
    fn decode_with(
        input: &'a [u8],
        _cx: &BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        <Self as Decodable<'a>>::decode(input)
    }
}
