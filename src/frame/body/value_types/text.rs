use derive_more::Deref;

use crate::frame::codec::{Decodable, Encodable};

pub type AsciiText<'a> = Text<'a, AsciiCodec>;
pub type Utf8Text<'a> = Text<'a, Utf8Codec>;

#[derive(Debug, Clone)]
pub struct AsciiCodec;

#[derive(Debug, Clone)]
pub struct Utf8Codec;

#[derive(Debug, Clone, Deref)]
#[repr(transparent)]
pub struct Text<'a, C> {
    #[deref]
    inner: &'a str,
    _phantom: std::marker::PhantomData<C>,
}

impl<'a, C: TextCodec> Decodable<'a> for Text<'a, C> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        let (rem, text) = C::decode(input)?;

        Ok((
            rem,
            Self {
                inner: text,
                _phantom: Default::default(),
            },
        ))
    }
}

impl<C: TextCodec> Encodable for Text<'_, C> {
    fn encode<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        writer.write_all(self.inner.as_bytes())?;

        Ok(())
    }
}

pub trait TextCodec {
    fn decode(input: &[u8]) -> nom::IResult<&[u8], &str>;
    fn encode(
        input: &str,
        writer: &mut impl std::io::Write,
    ) -> std::io::Result<()>;
}

impl TextCodec for AsciiCodec {
    fn decode(input: &[u8]) -> nom::IResult<&[u8], &str> {
        use nom::combinator::{map, rest, verify};

        let (r, text) = map(verify(rest, <[u8]>::is_ascii), |buf| unsafe {
            std::str::from_utf8_unchecked(buf)
        })(input)?;

        Ok((r, text))
    }

    fn encode(
        input: &str,
        writer: &mut impl std::io::Write,
    ) -> std::io::Result<()> {
        writer.write_all(input.as_bytes())?;

        Ok(())
    }
}

impl TextCodec for Utf8Codec {
    fn decode(input: &[u8]) -> nom::IResult<&[u8], &str> {
        use nom::combinator::{map_res, rest};

        let (r, text) = map_res(rest, std::str::from_utf8)(input)?;

        Ok((r, text))
    }

    fn encode(
        input: &str,
        writer: &mut impl std::io::Write,
    ) -> std::io::Result<()> {
        writer.write_all(input.as_bytes())?;

        Ok(())
    }
}
