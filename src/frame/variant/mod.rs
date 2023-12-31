mod cancel;
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

use derive_more::From;
use recode::util::EncoderExt;

use super::*;

#[derive(Debug, From)]
pub enum FrameVariant {
    Setup(Setup),
    Error(Error),
    Lease(Lease),
    Keepalive(Keepalive),
    RequestResponse(RequestResponse),
    RequestFNF(RequestFNF),
    RequestStream(RequestStream),
    RequestChannel(RequestChannel),
    RequestN(RequestN),
    Cancel(Cancel),
    Payload(Payload),
    MetadataPush(MetadataPush),
    Resume(Resume),
    ResumeOk(ResumeOk),
    Ext(Ext),
}

impl recode::Decoder<Buffer> for FrameVariant {
    type Error = crate::Error;

    fn decode(buf: &mut Buffer) -> Result<Self, crate::Error> {
        let h = buf.context();

        if h.frame_type()
            .flags_mask()
            .complement()
            .intersects(h.flags())
        {
            return Err(crate::Error::UnexpectedFlag {
                flag: h.flags(),
                frame_type: h.frame_type(),
                message: "unexpected flags detected",
            });
        }

        #[inline(always)]
        fn decode<T>(buf: &mut Buffer) -> Result<FrameVariant, crate::Error>
        where
            T: Decoder<Buffer, Error = crate::Error> + Into<FrameVariant>,
        {
            T::decode(buf).map(Into::into)
        }

        match h.frame_type() {
            | FrameType::Setup => decode::<Setup>(buf),
            | FrameType::Error => decode::<Error>(buf),
            | FrameType::Lease => decode::<Lease>(buf),
            | FrameType::Keepalive => decode::<Keepalive>(buf),
            | FrameType::RequestResponse => decode::<RequestResponse>(buf),
            | FrameType::RequestFNF => decode::<RequestFNF>(buf),
            | FrameType::RequestStream => decode::<RequestStream>(buf),
            | FrameType::RequestChannel => decode::<RequestChannel>(buf),
            | FrameType::RequestN => decode::<RequestN>(buf),
            | FrameType::Cancel => decode::<Cancel>(buf),
            | FrameType::Payload => decode::<Payload>(buf),
            | FrameType::MetadataPush => decode::<MetadataPush>(buf),
            | FrameType::Resume => decode::<Resume>(buf),
            | FrameType::ResumeOk => decode::<ResumeOk>(buf),
            | FrameType::Ext => decode::<Ext>(buf),
            | FrameType::Other(_) => {
                Err(crate::Error::UnsupportedFrameType(h.frame_type()))
            }
        }
    }
}

impl recode::Encoder<BufferMut> for FrameVariant {
    type Error = crate::Error;

    #[inline]
    fn encode(item: &Self, buf: &mut BufferMut) -> Result<(), crate::Error> {
        match item {
            | FrameVariant::Setup(v) => v.encode_to(buf),
            | FrameVariant::Error(v) => v.encode_to(buf),
            | FrameVariant::Lease(v) => v.encode_to(buf),
            | FrameVariant::Keepalive(v) => v.encode_to(buf),
            | FrameVariant::RequestResponse(v) => v.encode_to(buf),
            | FrameVariant::RequestFNF(v) => v.encode_to(buf),
            | FrameVariant::RequestStream(v) => v.encode_to(buf),
            | FrameVariant::RequestChannel(v) => v.encode_to(buf),
            | FrameVariant::RequestN(v) => v.encode_to(buf),
            | FrameVariant::Cancel(v) => v.encode_to(buf),
            | FrameVariant::Payload(v) => v.encode_to(buf),
            | FrameVariant::MetadataPush(v) => v.encode_to(buf),
            | FrameVariant::Resume(v) => v.encode_to(buf),
            | FrameVariant::ResumeOk(v) => v.encode_to(buf),
            | FrameVariant::Ext(v) => v.encode_to(buf),
        }
    }

    #[inline]
    fn size_of(item: &Self, buf: &BufferMut) -> usize {
        match item {
            | FrameVariant::Setup(v) => v.size(buf),
            | FrameVariant::Error(v) => v.size(buf),
            | FrameVariant::Lease(v) => v.size(buf),
            | FrameVariant::Keepalive(v) => v.size(buf),
            | FrameVariant::RequestResponse(v) => v.size(buf),
            | FrameVariant::RequestFNF(v) => v.size(buf),
            | FrameVariant::RequestStream(v) => v.size(buf),
            | FrameVariant::RequestChannel(v) => v.size(buf),
            | FrameVariant::RequestN(v) => v.size(buf),
            | FrameVariant::Cancel(v) => v.size(buf),
            | FrameVariant::Payload(v) => v.size(buf),
            | FrameVariant::MetadataPush(v) => v.size(buf),
            | FrameVariant::Resume(v) => v.size(buf),
            | FrameVariant::ResumeOk(v) => v.size(buf),
            | FrameVariant::Ext(v) => v.size(buf),
        }
    }
}
