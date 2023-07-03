use recode::{util::EncoderExt, Decoder, Encoder};

#[derive(Debug, recode::Recode)]
#[recode(error = "crate::Error")]
pub struct Error {
    pub code: ErrorCode,
    pub data: recode::codec::Utf8,
}

#[derive(Debug, Clone)]
#[from_to_repr::from_to_other(base_type = u32)]
pub enum ErrorCode {
    InvalidSetup = 0x00000001,
    UnsupportedSetup = 0x00000002,
    RejectSetup = 0x00000003,
    RejectResume = 0x00000004,
    ConnectionError = 0x00000101,
    ConnectionClose = 0x00000102,
    ApplicationError = 0x00000201,
    Rejected = 0x00000202,
    Canceled = 0x00000203,
    Invalid = 0x00000204,
    Other(u32),
}

impl<B> Decoder<B> for ErrorCode
where
    B: recode::bytes::Buf,
{
    type Error = crate::Error;

    fn decode(buf: &mut B) -> Result<Self, Self::Error> {
        u32::decode(buf)
            .map(Self::from_base_type)
            .map_err(Into::into)
    }
}

impl<B> Encoder<B> for ErrorCode
where
    B: recode::bytes::BufMut,
{
    type Error = crate::Error;

    fn encode(item: &Self, buf: &mut B) -> Result<(), Self::Error> {
        item.to_base_type().encode_to(buf).map_err(Into::into)
    }
}
