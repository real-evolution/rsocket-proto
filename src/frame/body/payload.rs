use derive_more::From;

use super::{codec::BodyCodec, Data, PrefixedMetadata};
use crate::error::RSocketResult;
use crate::frame::codec::{self, chained};
use crate::frame::{Flags, FrameHeader};

#[derive(Debug, Clone, From)]
pub struct Payload<'a> {
    pub metadata: Option<PrefixedMetadata<'a>>,
    pub data: Option<Data<'a>>,
}

impl<'a> BodyCodec<'a> for Payload<'a> {
    fn decode(
        input: &'a [u8],
        cx: &codec::ParseContext<'a>,
    ) -> nom::IResult<&'a [u8], Self> {
        chained(move |m| {
            Ok(Self {
                metadata: m.next_with(cx)?,
                data: m.next_with(cx)?,
            })
        })(input)
    }

    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }

    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header
            .validate()
            .flags_match_mask(
                Flags::METADATA | Flags::FOLLOW | Flags::COMPLETE | Flags::NEXT,
            )?
            .done()
    }
}
