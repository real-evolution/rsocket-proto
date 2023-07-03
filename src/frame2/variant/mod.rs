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
    Ext(Ext),
}

impl recode::Decoder<Buffer> for FrameVariant {
    type Error = crate::Error;

    fn decode(buf: &mut Buffer) -> Result<Self, crate::Error> {
        let h = buf.header();

        if h.frame_type()
            .flags_mask()
            .complement()
            .intersects(h.flags())
        {
            return Err(crate::Error::ProtocolViolation(
                "unexpected flags detected",
            ));
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
            | FrameType::Ext => decode::<Ext>(buf),
            | _ => unreachable!(),
        }
    }
}

impl recode::Encoder<BufferMut> for FrameVariant {
    type Error = crate::Error;

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
            | FrameVariant::Ext(v) => v.encode_to(buf),
        }
    }
}
