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

pub(super) use codec::*;

use derive_more::From;

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

impl<'a> ContextDecodable<'a, &BodyDecodeContext> for FrameBody<'a> {
    fn decode_with(
        input: &'a [u8],
        cx: &BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        use crate::frame::FrameType;

        decode_chained(move |m| {
            Ok(match cx.header.frame_type {
                | FrameType::Setup => m.next_with::<Setup, _>(cx)?.into(),
                | FrameType::Lease => m.next_with::<Lease, _>(cx)?.into(),
                | FrameType::Keepalive => m.next::<Keepalive>()?.into(),

                | FrameType::RequestResponse => {
                    m.next_with::<RequestResponse, _>(cx)?.into()
                }
                | FrameType::RequestFNF => {
                    m.next_with::<RequestFNF, _>(cx)?.into()
                }
                | FrameType::RequestStream => {
                    m.next_with::<RequestStream, _>(cx)?.into()
                }
                | FrameType::RequestChannel => {
                    m.next_with::<RequestChannel, _>(cx)?.into()
                }
                | FrameType::RequestN => m.next::<RequestN>()?.into(),
                | FrameType::Cancel => m.next::<Cancel>()?.into(),
                | FrameType::Payload => m.next_with::<Payload, _>(cx)?.into(),
                | FrameType::Error => m.next::<Error>()?.into(),
                | FrameType::MetadataPush => m.next::<MetadataPush>()?.into(),
                | FrameType::Resume => m.next::<Resume>()?.into(),
                | FrameType::ResumeOk => m.next::<ResumeOk>()?.into(),
                | FrameType::Other(_) => todo!(),
            })
        })(input)
    }
}
