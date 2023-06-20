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

impl<'a> BodyCodec<'a> for FrameBody<'a> {
    fn decode(
        header: &super::FrameHeader,
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self> {
        match header.frame_type {
            | FrameType::Setup => decode_body::<Setup>(header, input),
            | FrameType::Lease => decode_body::<Lease>(header, input),
            | FrameType::Keepalive => decode_body::<Keepalive>(header, input),
            | FrameType::RequestResponse => {
                decode_body::<RequestResponse>(header, input)
            }
            | FrameType::RequestFNF => decode_body::<RequestFNF>(header, input),
            | FrameType::RequestStream => {
                decode_body::<RequestStream>(header, input)
            }
            | FrameType::RequestChannel => {
                decode_body::<RequestChannel>(header, input)
            }
            | FrameType::RequestN => {
                decode_body::<RequestChannel>(header, input)
            }
            | FrameType::Cancel => decode_body::<Cancel>(header, input),
            | FrameType::Payload => decode_body::<Payload>(header, input),
            | FrameType::Error => decode_body::<Error>(header, input),
            | FrameType::MetadataPush => {
                decode_body::<MetadataPush>(header, input)
            }
            | FrameType::Resume => decode_body::<Resume>(header, input),
            | FrameType::ResumeOk => decode_body::<ResumeOk>(header, input),
            | FrameType::Other(_) => unreachable!(),
        }
    }

    fn encode<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            | FrameBody::Setup(b) => b.encode(writer),
            | FrameBody::Error(b) => b.encode(writer),
            | FrameBody::Lease(b) => b.encode(writer),
            | FrameBody::Keepalive(b) => b.encode(writer),
            | FrameBody::RequestResponse(b) => b.encode(writer),
            | FrameBody::RequestFNF(b) => b.encode(writer),
            | FrameBody::RequestStream(b) => b.encode(writer),
            | FrameBody::RequestChannel(b) => b.encode(writer),
            | FrameBody::RequestN(b) => b.encode(writer),
            | FrameBody::Cancel(b) => b.encode(writer),
            | FrameBody::Payload(b) => b.encode(writer),
            | FrameBody::MetadataPush(b) => b.encode(writer),
            | FrameBody::Ext(b) => b.encode(writer),
            | FrameBody::Resume(b) => b.encode(writer),
            | FrameBody::ResumeOk(b) => b.encode(writer),
        }
    }
}

fn decode_body<'a, C>(
    header: &super::FrameHeader,
    input: &'a [u8],
) -> nom::IResult<&'a [u8], FrameBody<'a>>
where
    C: BodyCodec<'a> + Into<FrameBody<'a>>,
{
    let (rest, body) = C::decode(header, input)?;

    Ok((rest, body.into()))
}
