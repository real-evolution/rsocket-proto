use derive_more::Deref;

use crate::frame::codec::{ContextDecodable, Decodable, Encodable};
use crate::frame::{BodyDecodeContext, Flags};

pub type PrefixedMetadata<'a> = Metadata<'a, true>;
pub type RestMetadata<'a> = Metadata<'a, false>;

#[derive(Debug, Clone, Deref)]
#[repr(transparent)]
pub struct Metadata<'a, const HAS_LEN: bool>(&'a [u8]);

impl<'a, const HAS_LEN: bool> Decodable<'a> for Metadata<'a, HAS_LEN> {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        use nom::combinator::rest;
        use nom::multi::length_data;
        use nom::number::complete::be_u24;

        let (rest, metadata) = if HAS_LEN {
            length_data(be_u24)(input)
        } else {
            rest(input)
        }?;

        Ok((rest, Self(metadata)))
    }
}

impl<'a, const HAS_LEN: bool> ContextDecodable<'a, &BodyDecodeContext>
    for Option<Metadata<'a, HAS_LEN>>
{
    fn decode_with(
        input: &'a [u8],
        cx: &BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        let flags = cx.header.flags;

        if flags.contains(Flags::METADATA) {
            return Ok((input, None));
        }

        let (rest, metadata) = Decodable::decode(input)?;

        Ok((rest, Some(metadata)))
    }
}

impl<const HAS_LEN: bool> Encodable for Metadata<'_, HAS_LEN> {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        use byteorder::{WriteBytesExt, BE};

        if HAS_LEN {
            writer.write_u24::<BE>(self.0.len() as u32)?;
        }

        writer.write_all(self.0)?;

        Ok(writer)
    }
}
