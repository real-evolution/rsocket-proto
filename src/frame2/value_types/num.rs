use derive_more::Deref;
use recode::{util::EncoderExt, Decoder, Encoder};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Deref)]
pub struct NonZero<T>(T);

impl<B, T> Decoder<B> for NonZero<T>
where
    T: Decoder<B> + num::Num,
    crate::Error: From<T::Error>,
{
    type Error = crate::Error;

    fn decode(buf: &mut B) -> Result<Self, Self::Error> {
        let value = T::decode(buf)?;

        if value.is_zero() {
            return Err(crate::Error::ProtocolViolation(
                "field does not allow decoding from zro",
            ));
        }

        Ok(Self(value))
    }
}

impl<B, T> Encoder<B> for NonZero<T>
where
    T: Encoder<B>,
    crate::Error: From<T::Error>,
{
    type Error = crate::Error;

    #[inline]
    fn encode(item: &Self, buf: &mut B) -> Result<(), Self::Error> {
        item.encode_to(buf)
    }
}
