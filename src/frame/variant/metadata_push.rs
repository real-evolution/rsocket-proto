use derive_getters::Getters;
use recode::Recode;

#[derive(Debug, Clone, Getters, Recode)]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "super::Buffer"),
    encoder(buffer_type = "super::BufferMut")
)]
pub struct MetadataPush {
    pub(crate) metadata: super::Metadata,
}
