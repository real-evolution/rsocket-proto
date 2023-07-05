mod macros;

pub mod variant;

use super::*;

#[derive(Debug)]
pub struct FrameBuilder(pub(crate) ());

impl FrameBuilder {
    pub const fn stream_id(
        &self,
        stream_id: StreamId,
    ) -> variant::VariantBuilder {
        variant::VariantBuilder(stream_id)
    }
}
