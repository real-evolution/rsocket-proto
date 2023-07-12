use derive_getters::Getters;
use recode::Recode;

#[derive(Debug, Clone, Getters, Recode)]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "super::Buffer"),
    encoder(buffer_type = "super::BufferMut")
)]
pub struct RequestResponse {
    #[recode(with = "super::Metadata")]
    pub(crate) metadata: Option<super::Metadata>,
    pub(crate) data: super::Data,
}
