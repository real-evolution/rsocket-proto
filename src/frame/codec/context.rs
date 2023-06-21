use crate::frame::FrameHeader;

#[derive(Debug)]
pub(crate) struct ParseContext<'a> {
    pub(crate) header: FrameHeader,
    #[allow(unused)]
    pub(crate) raw: &'a [u8],
}
