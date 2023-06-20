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
mod util;
mod version;

pub use cancel::Cancel;
pub use error::{Error, ErrorCode};
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
pub use version::Version;

use derive_more::From;

use super::FrameType;
use crate::frame::body::codec::BodyCodec;

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

impl<'a> FrameBody<'a> {
    pub(super) fn decode(
        header: &super::FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        #[inline(always)]
        fn decode<'a, C>(
            header: &super::FrameHeader,
            input: &'a [u8],
        ) -> nom::IResult<&'a [u8], FrameBody<'a>>
        where
            C: BodyCodec<'a> + Into<FrameBody<'a>>,
        {
            let (rest, body) = C::decode(header, input)?;

            Ok((rest, body.into()))
        }

        match header.frame_type {
            | FrameType::Setup => decode::<Setup>(header, input),
            | FrameType::Lease => decode::<Lease>(header, input),
            | FrameType::Keepalive => decode::<Keepalive>(header, input),
            | FrameType::RequestResponse => {
                decode::<RequestResponse>(header, input)
            }
            | FrameType::RequestFNF => decode::<RequestFNF>(header, input),
            | FrameType::RequestStream => {
                decode::<RequestStream>(header, input)
            }
            | FrameType::RequestChannel => {
                decode::<RequestChannel>(header, input)
            }
            | FrameType::RequestN => decode::<RequestChannel>(header, input),
            | FrameType::Cancel => decode::<Cancel>(header, input),
            | FrameType::Payload => decode::<Payload>(header, input),
            | FrameType::Error => decode::<Error>(header, input),
            | FrameType::MetadataPush => decode::<MetadataPush>(header, input),
            | FrameType::Resume => decode::<Resume>(header, input),
            | FrameType::ResumeOk => decode::<ResumeOk>(header, input),
            | FrameType::Other(_) => unreachable!(),
        }
    }
}
