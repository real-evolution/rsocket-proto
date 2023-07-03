mod error;
mod setup;

pub use error::{Error, ErrorCode};
pub use setup::Setup;

use derive_more::From;
use recode::util::EncoderExt;

use super::*;

#[derive(Debug, From)]
pub enum FrameVariant {
    Setup(Setup),
    Error(Error),
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

        match h.frame_type() {
            | FrameType::Setup => Setup::decode(buf).map(Into::into),
            | FrameType::Error => Error::decode(buf).map(Into::into),
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
        }
    }
}
