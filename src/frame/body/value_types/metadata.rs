use derive_more::Deref;

use crate::frame::{codec, Flags};

pub type PrefixedMetadata<'a> = Metadata<'a, true>;
pub type RestMetadata<'a> = Metadata<'a, false>;

#[derive(Debug, Deref)]
#[repr(transparent)]
pub struct Metadata<'a, const HAS_LEN: bool>(&'a [u8]);

impl<'a, const HAS_LEN: bool> Metadata<'a, HAS_LEN> {
    pub(crate) fn decode_opt(
        cx: &codec::ParseContext<'a>,
    ) -> impl FnMut(
        &'a [u8],
    ) -> nom::IResult<&'a [u8], Option<Metadata<'a, HAS_LEN>>>
           + 'a {
        let flags = cx.header.flags;

        move |input| {
            if flags.contains(Flags::METADATA) {
                return Ok((input, None));
            }

            let (rest, metadata) = Self::decode(input)?;

            Ok((rest, Some(metadata)))
        }
    }

    pub(crate) fn decode(
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Metadata<'a, HAS_LEN>> {
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

    pub(crate) fn encode<'b, W: std::io::Write>(
        &self,
        writer: &'b mut W,
    ) -> std::io::Result<&'b mut W> {
        use byteorder::{WriteBytesExt, BE};

        if HAS_LEN {
            writer.write_u24::<BE>(self.0.len() as u32)?;
        }

        writer.write_all(self.0)?;

        Ok(writer)
    }
}
