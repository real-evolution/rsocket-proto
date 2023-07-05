#[derive(Debug, Clone, recode::Recode)]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "super::Buffer"),
    encoder(buffer_type = "super::BufferMut")
)]
pub struct Ext {
    pub(crate) extended_type: u32,
    #[recode(with = "super::Metadata")]
    pub(crate) metadata: Option<super::Metadata>,
    pub(crate) data: super::Data,
}
