use std::io::Write;

use derive_more::From;

use super::codec::BodyCodec;
use super::{Data, MimeType, NonZero, PrefixedMetadata, ResumeToken, Version};
use crate::error::RSocketResult;
use crate::frame::codec::{self, chained};
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone, From)]
pub struct Setup<'a> {
    pub version: Version,
    pub keepalive: NonZero<u32>,
    pub lifetime: NonZero<u32>,
    pub token: Option<ResumeToken<'a>>,
    pub mime_metadata: MimeType<'a>,
    pub mime_data: MimeType<'a>,
    pub metadata: Option<PrefixedMetadata<'a>>,
    pub data: Data<'a>,
}

impl<'a> BodyCodec<'a> for Setup<'a> {
    fn decode(
        input: &'a [u8],
        cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        chained(move |m| {
            Ok(Self {
                version: m.next()?,
                keepalive: m.next()?,
                lifetime: m.next()?,
                token: m.next_with(cx)?,
                mime_metadata: m.next()?,
                mime_data: m.next()?,
                metadata: m.next_with(cx)?,
                data: m.next()?,
            })
        })(input)
    }

    fn encode<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        use byteorder::{WriteBytesExt, BE};

        // version
        self.version.encode(writer)?;
        // keepalive
        writer.write_u32::<BE>(self.keepalive)?;
        // lifetime
        writer.write_u32::<BE>(self.lifetime)?;
        // token

        Ok(())
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::METADATA | Flags::RESUME | Flags::LEASE)?
            .in_stream(0)?
            .done()
    }
}
