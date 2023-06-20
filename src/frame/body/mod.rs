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

pub(crate) use codec::BodyCodec;

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
