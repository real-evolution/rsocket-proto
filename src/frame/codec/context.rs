use crate::frame::FrameHeader;

pub(crate) struct ParseContext<'a> {
    pub(crate) header: FrameHeader,
    pub(crate) raw: &'a [u8],
}
