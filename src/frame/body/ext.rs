use derive_more::From;

use super::util::chained;
use super::{codec::BodyCodec, Data, Number, PrefixedMetadata};
use crate::error::RSocketResult;
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone, From)]
pub struct Ext<'a> {
    pub extended_type: Number<u32>,
    pub metadata: Option<PrefixedMetadata<'a>>,
    pub data: Data<'a>,
}

impl<'a> BodyCodec<'a> for Ext<'a> {
    fn decode(
        input: &'a [u8],
        cx: &super::BodyDecodeContext,
    ) -> nom::IResult<&'a [u8], Self> {
        chained(move |m| {
            Ok(Self {
                extended_type: m.next()?,
                metadata: m.next_with(cx)?,
                data: m.next()?,
            })
        })(input)
    }

    fn encode<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.extended_type.encode(writer)?;
        if let Some(metadata) = &self.metadata {
            metadata.encode(writer)?;
        }
        self.data.encode(writer)?;

        Ok(())
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(Flags::IGNORE | Flags::METADATA)?
            .done()
    }
}
