#[derive(Debug, Clone, recode::Recode)]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "super::Buffer"),
    encoder(buffer_type = "super::BufferMut")
)]
pub struct Ext {
    pub extended_type: u32,
    pub metadata: Option<super::Metadata>,
    pub data: super::Data,
}
