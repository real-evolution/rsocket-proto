use super::codec::EmptyBody;
use crate::{error::RSocketResult, frame::FrameHeader};

#[derive(Debug, Clone, Default)]
pub struct Cancel;

impl EmptyBody for Cancel {
    fn validate_header(header: &FrameHeader) -> RSocketResult<()> {
        header.validate().has_empty_flags()?.done()
    }
}
