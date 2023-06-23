use std::io::Write;

use super::codec::BodyCodec;
use super::util::chained;
use super::{Data, MimeType, NonZero, PrefixedMetadata, ResumeToken, Version};
use crate::error::RSocketResult;
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone)]
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
        cx: &super::BodyDecodeContext,
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
        // version
        self.version.encode(writer)?;
        // keepalive
        self.keepalive.encode(writer)?;
        // lifetime
        self.lifetime.encode(writer)?;
        // token (if present)
        if let Some(token) = &self.token {
            token.encode(writer)?;
        }
        // mime metadata
        self.mime_metadata.encode(writer)?;
        // mime data
        self.mime_data.encode(writer)?;
        // metadata (if present)
        if let Some(metadata) = &self.metadata {
            metadata.encode(writer)?;
        }
        // data
        self.data.encode(writer)?;

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
